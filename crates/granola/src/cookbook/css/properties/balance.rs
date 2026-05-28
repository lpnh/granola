use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `balance` property value.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
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
