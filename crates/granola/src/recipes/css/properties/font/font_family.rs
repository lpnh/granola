use crate::prelude::*;

/// The recipe for the CSS `font-family` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-family)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_family = CssDeclaration::from(FontFamily).inherit();
///
/// assert_eq!(css_font_family.bake(), "font-family: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FontFamily;

impl DeclarationRecipe for FontFamily {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "font-family".into()
    }
}
