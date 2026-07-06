mod margin_block_end;
pub use margin_block_end::*;
mod margin_bottom;
pub use margin_bottom::*;
mod margin_inline_end;
pub use margin_inline_end::*;
mod margin_left;
pub use margin_left::*;
mod margin_right;
pub use margin_right::*;
mod margin_top;
pub use margin_top::*;

// margin_block
// margin_block_start
// margin_inline
// margin_inline_start

use crate::prelude::*;

/// The recipe for the CSS `margin` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_margin = CssDeclaration::from(Margin).content("0");
///
/// assert_eq!(css_margin.bake(), "margin: 0;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Margin;

impl DeclarationRecipe for Margin {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "margin".into()
    }
}
