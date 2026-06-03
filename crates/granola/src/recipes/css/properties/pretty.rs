use std::borrow::Cow;

use crate::prelude::*;

/// The `pretty` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_text_wrap: CssTextWrap<Pretty> = CssTextWrap::from_recipe();
///
/// assert_eq!(css_text_wrap.bake(), "text-wrap: pretty;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pretty;

impl TextWrapRecipe for Pretty {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "pretty".into();
    }
}
