use std::fs;
use std::io;
use std::path::Path;

use granola::daisyui::{self, Component};

pub(crate) const COMPONENTS: &[Component] = &[daisyui::btn::COMPONENT, daisyui::link::COMPONENT];

pub(crate) fn daisyui_safelist() -> Vec<&'static str> {
    let mut classes = Vec::new();
    for component in COMPONENTS {
        classes.push(component.base_class);
        for category in component.categories {
            for variant in category.variants {
                classes.push(variant.class_name);
            }
        }
    }
    classes
}

/// Writes every concrete class declared by Granola's daisyUI components.
pub fn write_daisyui_safelist(path: impl AsRef<Path>) -> io::Result<()> {
    let mut classes = daisyui_safelist();
    classes.sort_unstable();
    fs::write(path, classes.join("\n"))
}
