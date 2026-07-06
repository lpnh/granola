use crate::prelude::*;

/// The recipe for the CSS `font-style` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-style)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_style = CssDeclaration::from(FontStyle).content("italic");
///
/// assert_eq!(css_font_style.bake(), "font-style: italic;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FontStyle;

impl DeclarationRecipe for FontStyle {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "font-style".into()
    }
}
