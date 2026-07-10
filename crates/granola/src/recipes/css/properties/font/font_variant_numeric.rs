use crate::prelude::*;

/// The recipe for the CSS `font-variant-numeric` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-variant-numeric)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_variant_numeric = CssDeclaration::from(FontVariantNumeric).content("tabular-nums");
///
/// assert_eq!(
///     css_font_variant_numeric.bake(),
///     "font-variant-numeric: tabular-nums;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FontVariantNumeric;

impl DeclarationRecipe for FontVariantNumeric {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "font-variant-numeric".into()
    }
}
