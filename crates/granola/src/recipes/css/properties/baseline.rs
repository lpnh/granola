use crate::prelude::*;

/// The `baseline` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_vertical_align = CssVerticalAlign::from(Baseline);
///
/// assert_eq!(css_vertical_align.bake(), "vertical-align: baseline;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Baseline;

impl VerticalAlignRecipe for Baseline {
    fn content_recipe(content: &mut Self::Content) {
        *content = "baseline".into();
    }
}
