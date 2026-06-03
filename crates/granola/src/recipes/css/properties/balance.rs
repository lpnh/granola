use std::borrow::Cow;

use crate::prelude::*;

/// The `balance` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_text_wrap: CssTextWrap<Balance> = CssTextWrap::from_recipe();
///
/// assert_eq!(css_text_wrap.bake(), "text-wrap: balance;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Balance;

impl TextWrapRecipe for Balance {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "balance".into();
    }
}
