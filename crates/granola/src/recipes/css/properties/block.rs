use crate::prelude::*;

/// The `block` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display: CssDisplay<Block> = CssDisplay::from_cookbook();
///
/// assert_eq!(css_display.bake(), "display: block;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Block;

impl DisplayRecipe for Block {
    fn content_recipe(content: &mut Self::Content) {
        *content = "block".into();
    }
}
