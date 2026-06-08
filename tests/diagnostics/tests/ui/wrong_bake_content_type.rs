// A recipe whose `type Content` differs from `recipe_boilerplate`'s
// `Cow<'static, str>`.

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl SpanRecipe for BrokenRecipe {
    type Content = u32;

    recipe_boilerplate!();
}

fn main() {}
