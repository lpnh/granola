use crate::prelude::*;

/// The recipe for the CSS `padding-top` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-top)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_padding_top = CssDeclaration::from(PaddingTop).content("1rem");
///
/// assert_eq!(css_padding_top.bake(), "padding-top: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PaddingTop;

impl DeclarationRecipe for PaddingTop {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "padding-top".into()
    }
}
