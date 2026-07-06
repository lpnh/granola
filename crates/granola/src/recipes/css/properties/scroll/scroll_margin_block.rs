use crate::prelude::*;

/// The recipe for the CSS `scroll-margin-block` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/scroll-margin-block)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_scroll_margin_block = CssDeclaration::from(ScrollMarginBlock).content("5ex");
///
/// assert_eq!(css_scroll_margin_block.bake(), "scroll-margin-block: 5ex;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScrollMarginBlock;

impl DeclarationRecipe for ScrollMarginBlock {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "scroll-margin-block".into()
    }
}
