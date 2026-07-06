use crate::prelude::*;

/// The recipe for the CSS `font-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-size)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_size = CssDeclaration::from(FontSize).content("0.875rem");
///
/// assert_eq!(css_font_size.bake(), "font-size: 0.875rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FontSize;

impl DeclarationRecipe for FontSize {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "font-size".into()
    }
}
