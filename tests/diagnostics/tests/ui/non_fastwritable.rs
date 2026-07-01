// A recipe whose `type Content` is a plain struct that is not `FastWritable`.

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct Foo;

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl SpanRecipe for BrokenRecipe {
    type Content = Foo;

    fn bake_content(_content: Foo) -> Bake {
        Bake::default()
    }
}

fn main() {}
