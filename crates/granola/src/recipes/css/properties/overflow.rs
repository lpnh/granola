mod overflow_wrap;
pub use overflow_wrap::*;

// overflow_anchor
// overflow_block
// overflow_clip-margin
// overflow_inline
// overflow_x
// overflow_y

use crate::prelude::*;

/// The recipe for the CSS `overflow` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/overflow)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_overflow = CssDeclaration::from(Overflow).content("hidden");
///
/// assert_eq!(css_overflow.bake(), "overflow: hidden;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Overflow;

impl DeclarationRecipe for Overflow {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "overflow".into()
    }
}
