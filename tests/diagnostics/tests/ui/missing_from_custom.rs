// Same as `missing_from.rs`, but with a custom `FastWritable` type instead.

use askama::{FastWritable, Values};
use std::fmt;

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct Foo;

impl FastWritable for Foo {
    fn write_into(&self, dest: &mut dyn fmt::Write, _: &dyn Values) -> askama::Result<()> {
        Ok(dest.write_str("foo")?)
    }
}

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl PTag for BrokenRecipe {
    type Content = Foo;
}

fn main() {}
