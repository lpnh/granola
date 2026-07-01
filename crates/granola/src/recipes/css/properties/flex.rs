use crate::prelude::*;

/// The `flex` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display = CssDisplay::from(Flex);
///
/// assert_eq!(css_display.bake(), "display: flex;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Flex;

impl DisplayRecipe for Flex {
    recipe_boilerplate!(DisplayRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "flex".into();
    }
}
