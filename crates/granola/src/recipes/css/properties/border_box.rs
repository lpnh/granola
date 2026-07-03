use crate::prelude::*;

/// The `border-box` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_box_sizing = CssBoxSizing::from(BorderBox);
///
/// assert_eq!(css_box_sizing.bake(), "box-sizing: border-box;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderBox;

impl BoxSizingRecipe for BorderBox {
    recipe_boilerplate!(BoxSizingRecipe);

    fn content_recipe() -> Self::Content {
        "border-box".into()
    }
}
