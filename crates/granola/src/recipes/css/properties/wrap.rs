use crate::prelude::*;

/// The `wrap` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_flex_wrap = CssFlexWrap::from(Wrap);
///
/// assert_eq!(css_flex_wrap.bake(), "flex-wrap: wrap;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Wrap;

impl FlexWrapRecipe for Wrap {
    recipe_boilerplate!(FlexWrapRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "wrap".into();
    }
}
