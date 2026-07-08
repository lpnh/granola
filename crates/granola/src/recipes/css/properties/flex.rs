mod flex_direction;
pub use flex_direction::*;
mod flex_shrink;
pub use flex_shrink::*;
mod flex_wrap;
pub use flex_wrap::*;

// flex_basis
// flex_flow
// flex_grow

use crate::prelude::*;

/// The recipe for the CSS `flex` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_flex = CssDeclaration::from(Flex).content("1");
///
/// assert_eq!(css_flex.bake(), "flex: 1;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Flex;

impl DeclarationRecipe for Flex {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "flex".into()
    }
}
