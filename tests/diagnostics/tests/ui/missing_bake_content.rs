// An empty recipe.

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl SpanRecipe for BrokenRecipe {}

fn main() {}
