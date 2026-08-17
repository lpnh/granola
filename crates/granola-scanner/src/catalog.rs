use std::fs;
use std::io;
use std::path::Path;

use granola::daisyui::{self, Component};

pub(crate) const COMPONENTS: &[Component] = &[
    daisyui::btn::COMPONENT,
    daisyui::card::COMPONENT,
    daisyui::link::COMPONENT,
];

pub fn write_daisyui_safelist(path: impl AsRef<Path>) -> io::Result<()> {
    let mut classes = daisyui_safelist();
    classes.sort_unstable();
    fs::write(path, classes.join("\n"))
}

pub(crate) fn daisyui_safelist() -> Vec<&'static str> {
    let mut classes = Vec::new();

    for c in COMPONENTS {
        classes.push(c.base_class);
        classes.extend(c.parts.iter().map(|cp| cp.class_name));
        classes.extend(
            c.categories
                .iter()
                .flat_map(|cc| cc.variants.iter().map(|cv| cv.class_name)),
        );
    }

    classes
}
