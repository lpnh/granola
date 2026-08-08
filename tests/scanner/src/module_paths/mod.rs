use granola as g;

use g::{daisyui as ui, prelude::*};

pub fn module_alias() -> String {
    HtmlButton::from(ui::btn::Btn)
        .color(ui::btn::Color::Accent)
        .content("Accent")
        .bake()
}

pub fn crate_alias() -> String {
    HtmlButton::from(g::daisyui::btn::Btn)
        .style(g::daisyui::btn::Style::Outline)
        .content("Outline")
        .bake()
}

pub fn fully_qualified() -> String {
    HtmlButton::from(granola::daisyui::Btn)
        .content("Direct")
        .bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn module_crate_and_fully_qualified_paths() {
        crate::assert_safelist("module_paths", &["btn", "btn-accent", "btn-outline"]);
    }
}
