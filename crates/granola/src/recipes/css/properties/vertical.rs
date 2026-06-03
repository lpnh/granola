use crate::prelude::*;

/// The `vertical` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_resize: CssResize<Vertical> = CssResize::from_cookbook();
///
/// assert_eq!(css_resize.bake(), "resize: vertical;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vertical;

impl ResizeRecipe for Vertical {
    fn content_recipe(content: &mut Self::Content) {
        *content = "vertical".into();
    }
}
