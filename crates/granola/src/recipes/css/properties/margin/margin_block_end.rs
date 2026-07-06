use crate::prelude::*;

/// The recipe for the CSS `margin-block-end` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-block-end)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_margin_block_end = CssDeclaration::from(MarginBlockEnd).content("0");
///
/// assert_eq!(css_margin_block_end.bake(), "margin-block-end: 0;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarginBlockEnd;

impl DeclarationRecipe for MarginBlockEnd {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "margin-block-end".into()
    }
}
