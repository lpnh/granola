use crate::prelude::*;

/// The `break-word` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_overflow_wrap = CssOverflowWrap::from(BreakWord);
///
/// assert_eq!(css_overflow_wrap.bake(), "overflow-wrap: break-word;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BreakWord;

impl OverflowWrapRecipe for BreakWord {
    recipe_boilerplate!(OverflowWrapRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "break-word".into();
    }
}
