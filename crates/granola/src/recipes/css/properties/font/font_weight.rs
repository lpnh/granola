use crate::prelude::*;

/// The recipe for the CSS `font-weight` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-weight)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_weight = CssDeclaration::from(FontWeight).content("500");
///
/// assert_eq!(css_font_weight.bake(), "font-weight: 500;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FontWeight;

impl DeclarationRecipe for FontWeight {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "font-weight".into()
    }
}
