use crate::prelude::*;

/// The recipe for the CSS `font-feature-settings` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-feature-settings)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_feature_settings = CssDeclaration::from(FontFeatureSettings).inherit();
///
/// assert_eq!(
///     css_font_feature_settings.bake(),
///     "font-feature-settings: inherit;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FontFeatureSettings;

impl DeclarationRecipe for FontFeatureSettings {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "font-feature-settings".into()
    }
}
