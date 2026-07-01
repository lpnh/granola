use crate::prelude::*;

/// The `vertical` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_resize = CssResize::from(Vertical);
///
/// assert_eq!(css_resize.bake(), "resize: vertical;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Vertical;

impl ResizeRecipe for Vertical {
    recipe_boilerplate!(ResizeRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "vertical".into();
    }
}
