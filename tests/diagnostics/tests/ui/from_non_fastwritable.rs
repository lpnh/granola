// A recipe that overrides `type Content` with a `BakeFrom<T>` whose inner `T` does
// not implement `FastWritable`.

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct Foo;

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl PRecipe for BrokenRecipe {
    type Content = BakeFrom<Foo>;
}

fn main() {}
