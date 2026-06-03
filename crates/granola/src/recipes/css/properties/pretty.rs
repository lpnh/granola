use crate::prelude::*;

/// The `pretty` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_wrap: CssTextWrap<Pretty> = CssTextWrap::from_cookbook();
///
/// assert_eq!(css_text_wrap.bake(), "text-wrap: pretty;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pretty;

impl TextWrapRecipe for Pretty {
    fn content_recipe(content: &mut Self::Content) {
        *content = "pretty".into();
    }
}
