use crate::prelude::*;

/// The recipe for the CSS `outline-style` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-style)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_outline_style = CssDeclaration::from(OutlineStyle).content("solid");
///
/// assert_eq!(css_outline_style.bake(), "outline-style: solid;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OutlineStyle;

impl DeclarationRecipe for OutlineStyle {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "outline-style".into()
    }
}
