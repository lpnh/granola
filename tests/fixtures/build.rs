fn main() {
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
