// mod color_interpolation;
// mod color_interpolation_filters;
// mod color_scheme;

use crate::prelude::*;

/// The recipe for the CSS `color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/color)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_color = CssDeclaration::from(Color).content("rebeccapurple");
///
/// assert_eq!(css_color.bake(), "color: rebeccapurple;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Color;

impl DeclarationRecipe for Color {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "color".into()
    }
}
