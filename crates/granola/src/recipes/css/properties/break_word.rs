use std::borrow::Cow;

use crate::prelude::*;

/// The `break-word` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_overflow_wrap: CssOverflowWrap<BreakWord> = CssOverflowWrap::from_cookbook();
///
/// assert_eq!(css_overflow_wrap.bake(), "overflow-wrap: break-word;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BreakWord;

impl OverflowWrapRecipe for BreakWord {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "break-word".into();
    }
}
