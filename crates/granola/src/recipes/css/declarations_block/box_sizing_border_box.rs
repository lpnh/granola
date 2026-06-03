use crate::{recipes::*, prelude::*};

/// The `box-sizing: border-box;` declaration recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let declarations_block: CssDeclarationsBlock<BoxSizingBorderBox> =
///     CssDeclarationsBlock::from_cookbook();
///
/// assert_eq!(declarations_block.bake(), "box-sizing: border-box;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BoxSizingBorderBox;

impl DeclarationsBlockRecipe for BoxSizingBorderBox {
    fn declarations_recipe(declarations: &mut Vec<CssDeclaration>) {
        declarations.push(CssBoxSizing::<BorderBox>::from_cookbook().into());
    }
}
