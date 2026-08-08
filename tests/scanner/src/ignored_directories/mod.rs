pub mod ui;
use ui::Button;

use granola::prelude::*;

pub fn render() -> String {
    HtmlButton::from(Button).content("Button").bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ignored_directories() {
        crate::assert_safelist("ignored_directories", &["btn"]);
    }
}
