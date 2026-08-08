use granola::prelude::*;

pub mod ui;

use ui::{Button, ButtonColor};

pub fn render() -> String {
    HtmlButton::from(Button)
        .color(ButtonColor::Secondary)
        .content("Cancel")
        .bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aliases_and_local_reexports() {
        crate::assert_safelist("aliases", &["btn", "btn-secondary"]);
    }
}
