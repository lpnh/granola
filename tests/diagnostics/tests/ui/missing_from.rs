// A recipe that overrides `type Content` but does not provide a `From` impl to
// convert that content back into the element's default content type.
//
// Expected: the `BakeInto` diagnostic points here and asks for the `From` impl.

use granola::prelude::*;

#[derive(Default, Debug, Clone)]
struct BrokenRecipe;

impl PTag for BrokenRecipe {
    type Content = u32;
}

fn main() {}
