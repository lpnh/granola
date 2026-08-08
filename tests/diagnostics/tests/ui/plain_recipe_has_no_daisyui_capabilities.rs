use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct PlainRecipe;

impl DivRecipe for PlainRecipe {
    recipe_boilerplate!(DivRecipe);
}

fn main() {
    let _ = HtmlDiv::from(PlainRecipe).color(());
}
