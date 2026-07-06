mod padding_block;
pub use padding_block::*;
mod padding_inline_start;
pub use padding_inline_start::*;

// mod padding_block-end;
// mod padding_block-start;
// mod padding_bottom;
// mod padding_inline;
// mod padding_inline-end;
// mod padding_left;
// mod padding_right;
// mod padding_top;

use crate::prelude::*;

/// The recipe for the CSS `padding` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_padding = CssDeclaration::from(Padding).content("0.6em 1.2em");
///
/// assert_eq!(css_padding.bake(), "padding: 0.6em 1.2em;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Padding;

impl DeclarationRecipe for Padding {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "padding".into()
    }
}
