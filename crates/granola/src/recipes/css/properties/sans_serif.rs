use crate::prelude::*;

/// The `sans-serif` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_family = CssFontFamily::from(SansSerif);
///
/// assert_eq!(css_font_family.bake(), "font-family: sans-serif;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SansSerif;

impl FontFamilyRecipe for SansSerif {
    fn content_recipe(content: &mut Self::Content) {
        *content = "sans-serif".into();
    }
}
