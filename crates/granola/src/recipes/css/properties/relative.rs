use crate::prelude::*;

/// The `relative` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_position: CssPosition<Relative> = CssPosition::from_cookbook();
///
/// assert_eq!(css_position.bake(), "position: relative;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Relative;

impl PositionRecipe for Relative {
    fn content_recipe(content: &mut Self::Content) {
        *content = "relative".into();
    }
}
