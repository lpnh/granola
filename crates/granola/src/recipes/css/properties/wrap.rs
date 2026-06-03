use std::borrow::Cow;

use crate::prelude::*;

/// The `wrap` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_flex_wrap: CssFlexWrap<Wrap> = CssFlexWrap::from_recipe();
///
/// assert_eq!(css_flex_wrap.bake(), "flex-wrap: wrap;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Wrap;

impl FlexWrapRecipe for Wrap {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "wrap".into();
    }
}
