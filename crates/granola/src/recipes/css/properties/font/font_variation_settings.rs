use crate::prelude::*;

/// The recipe for the CSS `font-variation-settings` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-variation-settings)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_variation_settings = CssDeclaration::from(FontVariationSettings).inherit();
///
/// assert_eq!(
///     css_font_variation_settings.bake(),
///     "font-variation-settings: inherit;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FontVariationSettings;

impl DeclarationRecipe for FontVariationSettings {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "font-variation-settings".into()
    }
}
