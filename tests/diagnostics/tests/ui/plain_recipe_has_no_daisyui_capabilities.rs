use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct PlainRecipe;

impl DivRecipe for PlainRecipe {}

fn main() {
    let _ = HtmlDiv::from(PlainRecipe).color(());
}
