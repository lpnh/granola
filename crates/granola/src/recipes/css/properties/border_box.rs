use crate::prelude::*;

/// The `border-box` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_box_sizing: CssBoxSizing<BorderBox> = CssBoxSizing::from_cookbook();
///
/// assert_eq!(css_box_sizing.bake(), "box-sizing: border-box;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderBox;

impl BoxSizingRecipe for BorderBox {
    fn content_recipe(content: &mut Self::Content) {
        *content = "border-box".into();
    }
}
