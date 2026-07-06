use crate::prelude::*;

/// The recipe for the CSS `padding-inline-start` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-inline-start)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_padding_inline_start = CssDeclaration::from(PaddingInlineStart).content("20px");
///
/// assert_eq!(
///     css_padding_inline_start.bake(),
///     "padding-inline-start: 20px;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PaddingInlineStart;

impl DeclarationRecipe for PaddingInlineStart {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "padding-inline-start".into()
    }
}
