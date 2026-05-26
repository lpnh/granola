use granola::prelude::*;

#[derive(Default, Debug, Clone, PartialEq)]
struct SomeRecipe;

impl ButtonRecipe for SomeRecipe {
    fn global_aria_attrs_recipe(global_aria_attrs: &mut GlobalAriaAttrs) {
        global_aria_attrs.aria_label("Search");
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct NotARecipe;

fn main() {
    let _button: HtmlButton<(SomeRecipe, NotARecipe)> = HtmlButton::from_recipe();
}
