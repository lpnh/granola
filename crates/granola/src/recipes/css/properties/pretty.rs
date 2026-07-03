use crate::prelude::*;

/// The `pretty` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_wrap = CssTextWrap::from(Pretty);
///
/// assert_eq!(css_text_wrap.bake(), "text-wrap: pretty;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pretty;

impl TextWrapRecipe for Pretty {
    recipe_boilerplate!(TextWrapRecipe);

    fn content_recipe() -> Self::Content {
        "pretty".into()
    }
}
