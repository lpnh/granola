use crate::prelude::*;

/// The `middle` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_vertical_align = CssVerticalAlign::from(Middle);
///
/// assert_eq!(css_vertical_align.bake(), "vertical-align: middle;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Middle;

impl VerticalAlignRecipe for Middle {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "middle".into();
    }
}
