mod outline_color;
pub use outline_color::*;
mod outline_offset;
pub use outline_offset::*;
mod outline_style;
pub use outline_style::*;
mod outline_width;
pub use outline_width::*;

use crate::prelude::*;

/// The recipe for the CSS `outline` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_outline = CssDeclaration::from(Outline).content("auto");
///
/// assert_eq!(css_outline.bake(), "outline: auto;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Outline;

impl DeclarationRecipe for Outline {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "outline".into()
    }
}
