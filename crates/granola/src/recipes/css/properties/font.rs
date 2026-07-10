mod font_family;
pub use font_family::*;
mod font_feature_settings;
pub use font_feature_settings::*;
mod font_size;
pub use font_size::*;
mod font_style;
pub use font_style::*;
mod font_variant_numeric;
pub use font_variant_numeric::*;
mod font_variation_settings;
pub use font_variation_settings::*;
mod font_weight;
pub use font_weight::*;

use crate::prelude::*;

/// The recipe for the CSS `font` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font = CssDeclaration::from(Font).inherit();
///
/// assert_eq!(css_font.bake(), "font: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Font;

impl DeclarationRecipe for Font {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "font".into()
    }
}
