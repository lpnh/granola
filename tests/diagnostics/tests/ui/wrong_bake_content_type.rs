// A recipe whose `type Content` differs from `recipe_boilerplate`'s
// `Cow<'static, str>`.

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl SpanRecipe for BrokenRecipe {
    recipe_boilerplate!(SpanRecipe, u32);
}

fn main() {}
