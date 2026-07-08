mod border_bottom;
pub use border_bottom::*;
mod border_collapse;
pub use border_collapse::*;
mod border_inline_start;
pub use border_inline_start::*;
mod border_color;
pub use border_color::*;
mod border_radius;
pub use border_radius::*;
mod border_style;
pub use border_style::*;
mod border_top;
pub use border_top::*;
mod border_top_width;
pub use border_top_width::*;
mod border_width;
pub use border_width::*;

// mod border_block;
// mod border_block_color;
// mod border_block_end;
// mod border_block_end-color;
// mod border_block_end-style;
// mod border_block_end-width;
// mod border_block_start;
// mod border_block_start-color;
// mod border_block_start-style;
// mod border_block_start-width;
// mod border_block_style;
// mod border_block_width;
// mod border_bottom_color;
// mod border_bottom_left-radius;
// mod border_bottom_right-radius;
// mod border_bottom_style;
// mod border_bottom_width;
// mod border_collapse;
// mod border_end_end_radius;
// mod border_end_start_radius;
// mod border_image;
// mod border_image_outset;
// mod border_image_repeat;
// mod border_image_slice;
// mod border_image_source;
// mod border_image_width;
// mod border_inline;
// mod border_inline_color;
// mod border_inline_end;
// mod border_inline_end_color;
// mod border_inline_end_style;
// mod border_inline_end_width;
// mod border_inline_start;
// mod border_inline_start_color;
// mod border_inline_start_style;
// mod border_inline_start_width;
// mod border_inline_style;
// mod border_inline_width;
// mod border_left;
// mod border_left_color;
// mod border_left_style;
// mod border_left_width;
// mod border_right;
// mod border_right_color;
// mod border_right_style;
// mod border_right_width;
// mod border_spacing;
// mod border_start_end_radius;
// mod border_start_start_radius;
// mod border_top_color;
// mod border_top_left_radius;
// mod border_top_right_radius;
// mod border_top_style;

use crate::prelude::*;

/// The recipe for the CSS `border` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border = CssDeclaration::from(Border).content("none");
///
/// assert_eq!(css_border.bake(), "border: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Border;

impl DeclarationRecipe for Border {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border".into()
    }
}
