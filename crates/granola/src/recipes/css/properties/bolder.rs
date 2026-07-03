use crate::prelude::*;

/// The `bolder` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_weight = CssFontWeight::from(Bolder);
///
/// assert_eq!(css_font_weight.bake(), "font-weight: bolder;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bolder;

impl FontWeightRecipe for Bolder {
    recipe_boilerplate!(FontWeightRecipe);

    fn content_recipe() -> Self::Content {
        "bolder".into()
    }
}
