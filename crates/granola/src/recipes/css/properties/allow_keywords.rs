use crate::prelude::*;

/// The `allow-keywords` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_interpolate_size = CssInterpolateSize::from(AllowKeywords);
///
/// assert_eq!(
///     css_interpolate_size.bake(),
///     "interpolate-size: allow-keywords;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllowKeywords;

impl InterpolateSizeRecipe for AllowKeywords {
    recipe_boilerplate!(InterpolateSizeRecipe);

    fn content_recipe() -> Self::Content {
        "allow-keywords".into()
    }
}
