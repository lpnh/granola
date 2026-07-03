use crate::prelude::*;

/// The `block` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display = CssDisplay::from(Block);
///
/// assert_eq!(css_display.bake(), "display: block;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Block;

impl DisplayRecipe for Block {
    recipe_boilerplate!(DisplayRecipe);

    fn content_recipe() -> Self::Content {
        "block".into()
    }
}
