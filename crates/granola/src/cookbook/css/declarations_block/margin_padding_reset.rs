use crate::prelude::*;

/// The `margin: 0; padding: 0;` declaration recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let declarations_block: CssDeclarationsBlock<MarginPaddingReset> =
///     CssDeclarationsBlock::from_recipe();
///
/// assert_eq!(
///     declarations_block.bake(),
///     "margin: 0;
/// padding: 0;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarginPaddingReset;

impl DeclarationsBlockRecipe for MarginPaddingReset {
    fn declarations_recipe(declarations: &mut Vec<CssDeclaration>) {
        declarations.extend([
            CssMargin::<()>::new("0").into(),
            CssPadding::<()>::new("0").into(),
        ]);
    }
}
