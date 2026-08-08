pub mod middle;
pub mod public;
pub mod source;
use public::Button;

use granola::prelude::*;

pub fn render() -> String {
    HtmlButton::from(Button).content("Re-export").bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn chained_local_reexports() {
        crate::assert_safelist("reexports", &["btn"]);
    }
}
