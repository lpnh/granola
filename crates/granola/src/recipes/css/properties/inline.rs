use crate::prelude::*;

/// The `inline` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display: CssDisplay<Inline> = CssDisplay::from_cookbook();
///
/// assert_eq!(css_display.bake(), "display: inline;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Inline;

impl DisplayRecipe for Inline {
    fn content_recipe(content: &mut Self::Content) {
        *content = "inline".into();
    }
}
