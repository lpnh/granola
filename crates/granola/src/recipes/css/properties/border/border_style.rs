use crate::prelude::*;

/// The recipe for the CSS `border-style` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-style)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_style = CssDeclaration::from(BorderStyle).content("1.2rem");
///
/// assert_eq!(css_border_style.bake(), "border-style: 1.2rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderStyle;

impl DeclarationRecipe for BorderStyle {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border-style".into()
    }
}
