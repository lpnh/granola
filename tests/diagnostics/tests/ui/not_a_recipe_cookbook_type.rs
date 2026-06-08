use granola::{prelude::*, recipes::*};

#[derive(Default, Debug, Clone, PartialEq)]
struct SomeRecipe;

impl ButtonRecipe for SomeRecipe {
    recipe_boilerplate!();

    fn global_aria_attrs_recipe(global_aria_attrs: &mut GlobalAriaAttrs) {
        global_aria_attrs.aria_label("Search");
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct NotARecipe;

type Cookbook = cookbook_type![FormmethodGet, SomeRecipe, NotARecipe, Center];

fn main() {
    let _button = HtmlButton::<Cookbook>::from_cookbook();
}
