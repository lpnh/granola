use granola::prelude::*;

#[derive(Default, Debug, Clone, PartialEq)]
struct NotARecipe;

fn main() {
    let _button: HtmlButton<NotARecipe> = HtmlButton::from_cookbook();
}
