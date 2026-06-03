use std::borrow::Cow;

use crate::prelude::*;

/// The `allow-keywords` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_interpolate_size: CssInterpolateSize<AllowKeywords> = CssInterpolateSize::from_recipe();
///
/// assert_eq!(
///     css_interpolate_size.bake(),
///     "interpolate-size: allow-keywords;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllowKeywords;

impl InterpolateSizeRecipe for AllowKeywords {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "allow-keywords".into();
    }
}
