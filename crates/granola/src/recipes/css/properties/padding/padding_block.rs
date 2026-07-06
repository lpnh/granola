use crate::prelude::*;

/// The recipe for the CSS `padding-block` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-block)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_padding_block = CssDeclaration::from(PaddingBlock).content("0");
///
/// assert_eq!(css_padding_block.bake(), "padding-block: 0;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PaddingBlock;

impl DeclarationRecipe for PaddingBlock {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "padding-block".into()
    }
}
