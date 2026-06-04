use crate::prelude::*;

/// The `underline` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration = CssTextDecoration::from(Underline);
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: underline;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Underline;

impl TextDecorationRecipe for Underline {
    fn content_recipe(content: &mut Self::Content) {
        *content = "underline".into();
    }
}

impl WebkitTextDecorationRecipe for Underline {
    fn content_recipe(content: &mut Self::Content) {
        *content = "underline".into();
    }
}
