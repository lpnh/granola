use crate::prelude::*;

/// The recipe for the CSS `padding-inline` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-inline)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_padding_inline = CssDeclaration::from(PaddingInline).content("1rem");
///
/// assert_eq!(css_padding_inline.bake(), "padding-inline: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PaddingInline;

impl DeclarationRecipe for PaddingInline {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "padding-inline".into()
    }
}
