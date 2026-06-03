use crate::{recipes::*, prelude::*};

/// The `border: 0 solid;` declaration recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let declarations_block: CssDeclarationsBlock<BorderReset> = CssDeclarationsBlock::from_cookbook();
///
/// assert_eq!(declarations_block.bake(), "border: 0 solid;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderReset;

impl DeclarationsBlockRecipe for BorderReset {
    fn declarations_recipe(declarations: &mut Vec<CssDeclaration>) {
        declarations.push(CssBorder::<Solid>::new("0").into());
    }
}
