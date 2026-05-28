use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `block` property value.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_display: CssDisplay<Block> = CssDisplay::from_recipe();
///
/// assert_eq!(css_display.bake(), "display: block;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Block;

impl DisplayRecipe for Block {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "block".into();
    }
}
