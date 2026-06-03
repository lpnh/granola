use crate::prelude::*;

/// The `bolder` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_weight: CssFontWeight<Bolder> = CssFontWeight::from_cookbook();
///
/// assert_eq!(css_font_weight.bake(), "font-weight: bolder;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bolder;

impl FontWeightRecipe for Bolder {
    fn content_recipe(content: &mut Self::Content) {
        *content = "bolder".into();
    }
}
