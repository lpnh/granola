// A recipe whose `type Content` cannot map back into `recipe_boilerplate`'s
// default content type (`Bake`): `Celsius` satisfies the `Content`
// bounds but has no `Into<Bake>` impl.

use std::fmt;

use askama::{FastWritable, Values};
use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct Celsius(i32);

impl FastWritable for Celsius {
    fn write_into(&self, dest: &mut dyn fmt::Write, _: &dyn Values) -> askama::Result<()> {
        write!(dest, "{}°C", self.0)?;
        Ok(())
    }
}

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl SpanRecipe for BrokenRecipe {
    recipe_boilerplate!(SpanRecipe, Celsius);
}

fn main() {}
