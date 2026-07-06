mod position_anchor;
pub use position_anchor::*;
mod position_area;
pub use position_area::*;
mod position_try_fallbacks;
pub use position_try_fallbacks::*;

// position_try;
// position_try_order;
// position_visibility;

use crate::prelude::*;

/// The recipe for the CSS `position` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_position = CssDeclaration::from(Position).content("relative");
///
/// assert_eq!(css_position.bake(), "position: relative;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Position;

impl DeclarationRecipe for Position {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "position".into()
    }
}
