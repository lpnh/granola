use crate::prelude::*;

/// The `balance` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_wrap = CssTextWrap::from(Balance);
///
/// assert_eq!(css_text_wrap.bake(), "text-wrap: balance;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Balance;

impl TextWrapRecipe for Balance {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "balance".into();
    }
}
