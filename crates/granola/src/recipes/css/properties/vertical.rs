use std::borrow::Cow;

use crate::prelude::*;

/// The `vertical` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_resize: CssResize<Vertical> = CssResize::from_cookbook();
///
/// assert_eq!(css_resize.bake(), "resize: vertical;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vertical;

impl ResizeRecipe for Vertical {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "vertical".into();
    }
}
