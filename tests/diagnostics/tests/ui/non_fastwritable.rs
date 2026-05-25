// A recipe whose `type Content` is a plain struct that is neither
// `FastWritable` nor convertible into the default content type.

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct Foo;

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl PTag for BrokenRecipe {
    type Content = Foo;
}

fn main() {}
