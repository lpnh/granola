use std::fs;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{self, Command};

use anyhow::{Context, Result, bail};

const GITHUB: &str = "https://github.com";

const DEPENDENCIES_DIR: &str = "dependencies";
const GRANOLA_DIR: &str = "granola";

const TAILWIND_REPO: &str = "tailwindlabs/tailwindcss";
const TAILWIND_BINARY: &str = "tailwindcss-linux-x64";
const TAILWIND_CHECKSUMS: &str = "sha256sums.txt";
const TAILWIND_FILE: &str = "tailwindcss";

const DAISYUI_REPO: &str = "saadeghi/daisyui";
const DAISYUI_PLUGIN: &str = "daisyui.mjs";
const DAISYUI_THEME: &str = "daisyui-theme.mjs";

const INPUT_CSS_FILE: &str = "input.css";
const BUILD_RS_FILE: &str = "build.rs";
const GITIGNORE_FILE: &str = ".gitignore";
const OUTPUT_CSS_FILE: &str = "output.css";

const INPUT_CSS: &str = r#"@import "tailwindcss";

@source not "./dependencies/tailwindcss";
@source not "./dependencies/daisyui{,*}.mjs";

@plugin "./dependencies/daisyui.mjs";
@source "./granola/safelist";
"#;

const BUILD_RS: &str = r#"fn main() {
    let classes = granola_scanner::scan_dir("src").unwrap();
    granola_scanner::write_safelist("granola/safelist", &classes).unwrap();

    let status = std::process::Command::new("dependencies/tailwindcss")
        .args(["-i", "input.css", "-o", "output.css"])
        .status()
        .unwrap_or_else(|e| {
            eprintln!("failed to run tailwindcss: {e}");
            eprintln!("run `granola init` to download it");
            std::process::exit(1);
        });
    if !status.success() {
        std::process::exit(1);
    }
}
"#;

const GITIGNORE_ENTRIES: &[&str] = &["dependencies/", "granola/", OUTPUT_CSS_FILE];

fn release_latest(repo: &str) -> String {
    format!("{GITHUB}/{repo}/releases/latest")
}

fn release_asset(repo: &str, tag: &str, asset: &str) -> String {
    format!("{GITHUB}/{repo}/releases/download/{tag}/{asset}")
}

fn main() -> Result<()> {
    let command = std::env::args().nth(1);

    match command.as_deref() {
        Some("init") => cmd_init()?,
        Some("update") => cmd_update()?,
        Some("--help" | "-h") | None => usage(),
        Some(other) => {
            usage();
            bail!("unknown command: {other}");
        }
    }
    Ok(())
}

fn usage() {
    println!("Usage: granola <command>");
    println!();
    println!("Commands:");
    println!("  init    Download Tailwind CSS + daisyUI, scaffold project files");
    println!("  update  Re-download Tailwind CSS + daisyUI to latest versions");
}

fn cmd_init() -> Result<()> {
    let tw = resolve_latest_tag(TAILWIND_REPO)?;
    let daisy = resolve_latest_tag(DAISYUI_REPO)?;
    download_tailwind(&tw)?;
    download_daisyui(&daisy)?;
    ensure_granola_dir()?;
    write_input_css()?;
    write_build_rs()?;
    update_gitignore()?;
    eprintln!("  done");
    eprintln!("  build.rs generates output.css on the next `cargo build`");
    Ok(())
}

fn cmd_update() -> Result<()> {
    let tw_latest = resolve_latest_tag(TAILWIND_REPO)?;
    let daisy_latest = resolve_latest_tag(DAISYUI_REPO)?;
    let tw_current = installed_tailwind_version();
    let daisy_current = installed_daisyui_version();

    if tw_current.as_deref() == Some(tw_latest.as_str())
        && daisy_current.as_deref() == Some(daisy_latest.as_str())
    {
        eprintln!("  already up to date (tailwindcss {tw_latest}, daisyui {daisy_latest})");
        return Ok(());
    }

    if tw_current.as_deref() != Some(tw_latest.as_str()) {
        eprintln!(
            "  tailwindcss {} -> {tw_latest}",
            tw_current.as_deref().unwrap_or("?")
        );
        download_tailwind(&tw_latest)?;
    }
    if daisy_current.as_deref() != Some(daisy_latest.as_str()) {
        eprintln!(
            "  daisyui {} -> {daisy_latest}",
            daisy_current.as_deref().unwrap_or("?")
        );
        download_daisyui(&daisy_latest)?;
    }

    eprintln!("  done");
    Ok(())
}

fn ensure_dependencies_dir() -> Result<()> {
    fs::create_dir_all(DEPENDENCIES_DIR)
        .with_context(|| format!("failed to create {DEPENDENCIES_DIR} directory"))
}

fn ensure_granola_dir() -> Result<()> {
    fs::create_dir_all(GRANOLA_DIR)
        .with_context(|| format!("failed to create {GRANOLA_DIR} directory"))
}

fn dependency_path(file: &str) -> PathBuf {
    Path::new(DEPENDENCIES_DIR).join(file)
}

fn staged_path(file: &str) -> PathBuf {
    Path::new(DEPENDENCIES_DIR).join(format!("{file}.{}.temp", process::id()))
}

fn installed_tailwind_version() -> Option<String> {
    let output = Command::new(dependency_path(TAILWIND_FILE))
        .arg("--help")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    stdout
        .split_whitespace()
        .chain(stderr.split_whitespace())
        .find(|token| {
            token
                .strip_prefix('v')
                .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit()))
        })
        .map(String::from)
}

fn installed_daisyui_version() -> Option<String> {
    let text = fs::read_to_string(dependency_path(DAISYUI_PLUGIN)).ok()?;
    text.lines().find_map(|line| {
        let rest = line.strip_prefix("var version = \"")?;
        let version = rest.strip_suffix("\";")?.trim();
        Some(format!("v{version}"))
    })
}

fn resolve_latest_tag(repo: &str) -> Result<String> {
    let url = release_latest(repo);
    let output = Command::new("curl")
        .args(["-fsIL", "-o", "/dev/null", "-w", "%{url_effective}", &url])
        .output()
        .with_context(|| format!("failed to run curl to resolve latest release for {repo}"))?;
    if !output.status.success() {
        bail!("failed to resolve latest release for {repo}");
    }
    let resolved = String::from_utf8_lossy(&output.stdout);
    let tag = resolved.rsplit('/').next().unwrap_or_default().trim();
    if tag.is_empty() || tag == "latest" {
        bail!("unexpected release URL for {repo}: {}", resolved.trim());
    }
    Ok(tag.to_owned())
}

fn download_tailwind(tag: &str) -> Result<()> {
    ensure_dependencies_dir()?;
    let staged = download_to(
        TAILWIND_FILE,
        &release_asset(TAILWIND_REPO, tag, TAILWIND_BINARY),
    )?;
    verify_checksum(
        &staged,
        TAILWIND_FILE,
        TAILWIND_BINARY,
        &release_asset(TAILWIND_REPO, tag, TAILWIND_CHECKSUMS),
    )?;
    set_executable(&staged, TAILWIND_FILE)?;
    install_staged_file(&staged, TAILWIND_FILE)?;
    Ok(())
}

fn download_daisyui(tag: &str) -> Result<()> {
    ensure_dependencies_dir()?;
    let staged_plugin = download_to(
        DAISYUI_PLUGIN,
        &release_asset(DAISYUI_REPO, tag, DAISYUI_PLUGIN),
    )?;
    let staged_theme = download_to(
        DAISYUI_THEME,
        &release_asset(DAISYUI_REPO, tag, DAISYUI_THEME),
    )?;
    install_staged_file(&staged_plugin, DAISYUI_PLUGIN)?;
    install_staged_file(&staged_theme, DAISYUI_THEME)?;
    Ok(())
}

fn verify_checksum(staged: &Path, file: &str, entry_name: &str, checksums_url: &str) -> Result<()> {
    let output = Command::new("curl")
        .args(["-fsSL", checksums_url])
        .output()
        .with_context(|| format!("failed to fetch checksums from {checksums_url}"))?;
    if !output.status.success() {
        let _ = fs::remove_file(staged);
        bail!("failed to download checksums from {checksums_url}");
    }
    let text = String::from_utf8(output.stdout)
        .with_context(|| format!("invalid UTF-8 in checksums from {checksums_url}"))?;
    let expected = text.lines().find_map(|line| {
        let mut parts = line.split_whitespace();
        let hash = parts.next()?;
        let name = parts.next()?.trim_start_matches("./");
        (name == entry_name).then(|| hash.to_owned())
    });
    let Some(expected) = expected else {
        let _ = fs::remove_file(staged);
        bail!("no checksum for {file} in {checksums_url}");
    };
    let actual = sha256sum(staged, file)?;
    if actual != expected {
        let _ = fs::remove_file(staged);
        bail!("checksum mismatch for {file} (expected: {expected}, got: {actual})");
    }
    eprintln!("  verified {file} checksum");
    Ok(())
}

fn sha256sum(path: &Path, file: &str) -> Result<String> {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .with_context(|| format!("failed to run sha256sum for {file}"))?;
    if !output.status.success() {
        bail!("sha256sum failed for {file}");
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let hash = text
        .split_whitespace()
        .next()
        .ok_or_else(|| anyhow::anyhow!("sha256sum produced no output for {file}"))?;
    Ok(hash.to_owned())
}

fn download_to(file: &str, url: &str) -> Result<PathBuf> {
    let staged = staged_path(file);
    eprintln!("  downloading {file}");
    let status = Command::new("curl")
        .arg("-fsSL")
        .arg("-o")
        .arg(&staged)
        .arg(url)
        .status()
        .with_context(|| format!("failed to run curl for {file}"))?;
    if !status.success() {
        let _ = fs::remove_file(&staged);
        bail!("download failed: {file}");
    }
    Ok(staged)
}

fn install_staged_file(staged: &Path, file: &str) -> Result<()> {
    let destination = dependency_path(file);
    fs::rename(staged, destination).with_context(|| format!("failed to install {file}"))?;
    Ok(())
}

fn set_executable(path: &Path, file: &str) -> Result<()> {
    let meta = fs::metadata(path).with_context(|| format!("cannot read metadata for {file}"))?;
    let mut perms = meta.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)
        .with_context(|| format!("cannot set permissions on {file}"))?;
    Ok(())
}

fn read_existing_file(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(Some(contents)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("failed to read {}", path.display())),
    }
}

fn write_input_css() -> Result<()> {
    let path = Path::new(INPUT_CSS_FILE);
    if read_existing_file(path)?.is_some() {
        eprintln!("  {INPUT_CSS_FILE} already exists, skipping");
        return Ok(());
    }
    fs::write(path, INPUT_CSS).context(format!("failed to write {INPUT_CSS_FILE}"))?;
    eprintln!("  wrote {INPUT_CSS_FILE}");
    Ok(())
}

fn write_build_rs() -> Result<()> {
    let path = Path::new(BUILD_RS_FILE);
    if let Some(existing) = read_existing_file(path)? {
        if existing == BUILD_RS {
            eprintln!("  {BUILD_RS_FILE} up to date");
        } else {
            eprintln!("  {BUILD_RS_FILE} exists with different content, skipping");
        }
        return Ok(());
    }
    fs::write(path, BUILD_RS).context(format!("failed to write {BUILD_RS_FILE}"))?;
    eprintln!("  wrote {BUILD_RS_FILE}");
    Ok(())
}

fn update_gitignore() -> Result<()> {
    let path = Path::new(GITIGNORE_FILE);
    let existing = read_existing_file(path)?.unwrap_or_default();
    let missing = missing_gitignore_entries(&existing);

    if missing.is_empty() {
        eprintln!("  {GITIGNORE_FILE} up to date");
        return Ok(());
    }

    let additions = gitignore_additions(&existing, &missing);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .context(format!("failed to open {GITIGNORE_FILE}"))?;

    file.write_all(additions.as_bytes())
        .context(format!("failed to update {GITIGNORE_FILE}"))?;

    eprintln!("  updated {GITIGNORE_FILE}");
    Ok(())
}

fn missing_gitignore_entries(existing: &str) -> Vec<&'static str> {
    GITIGNORE_ENTRIES
        .iter()
        .copied()
        .filter(|entry| !existing.lines().any(|line| line.trim() == *entry))
        .collect()
}

fn gitignore_additions(existing: &str, missing: &[&str]) -> String {
    let mut additions = String::new();
    if !existing.is_empty() && !existing.ends_with('\n') {
        additions.push('\n');
    }
    for entry in missing {
        additions.push_str(entry);
        additions.push('\n');
    }
    additions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gitignore_entries_ignore_whitespace_and_duplicates() {
        let existing = "target/\n  dependencies/  \n# other\n";

        assert_eq!(
            missing_gitignore_entries(existing),
            vec!["granola/", OUTPUT_CSS_FILE]
        );
    }

    #[test]
    fn gitignore_additions_keep_existing_content_separate() {
        let existing = "target/";
        let missing = ["dependencies/"];

        assert_eq!(gitignore_additions(existing, &missing), "\ndependencies/\n");
    }

    #[test]
    fn staged_files_are_visible_and_end_in_temp() {
        let staged = staged_path(DAISYUI_PLUGIN);
        let name = staged.file_name().unwrap().to_str().unwrap();

        assert!(name.starts_with("daisyui.mjs."));
        assert!(name.ends_with(".temp"));
    }
}
