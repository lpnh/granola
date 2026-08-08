pub mod aliases;
pub mod comments_and_strings;
pub mod glob_imports;
pub mod ignored_directories;
pub mod lexical_restoration;
pub mod link;
pub mod local_macro;
pub mod macro_alias;
pub mod macro_tokens;
pub mod module_paths;
pub mod non_component_symbols;
pub mod rebinding;
pub mod reexports;
pub mod shadowing_unrelated;
pub mod typed_button;
pub mod unrelated_symbols;

#[cfg(test)]
use std::path::Path;

#[path = "relative_modules/current.rs"]
pub mod relative_current;
#[path = "relative_modules/order.rs"]
pub mod relative_order;
#[path = "relative_modules/parent.rs"]
pub mod relative_parent;

#[cfg(test)]
pub(crate) fn assert_safelist(case: &str, expected: &[&str]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_classes(root.join("src").join(case), expected);
}

#[cfg(test)]
pub(crate) fn assert_safelist_file(case: &str, file: &str, expected: &[&str]) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_classes(root.join("src").join(case).join(file), expected);
}

#[cfg(test)]
fn assert_classes(path: impl AsRef<Path>, expected: &[&str]) {
    use std::collections::HashSet;

    let classes = granola_scanner::scan_dir(path).unwrap();
    let expected: HashSet<String> = expected.iter().copied().map(String::from).collect();

    assert_eq!(classes, expected);
}
