use crate::prelude::*;

/// The recipe for the CSS `padding-bottom` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-bottom)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_padding_bottom = CssDeclaration::from(PaddingBottom).content("1rem");
///
/// assert_eq!(css_padding_bottom.bake(), "padding-bottom: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PaddingBottom;

impl DeclarationRecipe for PaddingBottom {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "padding-bottom".into()
    }
}
