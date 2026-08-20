use crate::prelude::*;

/// The recipe for the CSS `aspect-ratio` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/aspect-ratio)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_aspect_ratio = CssDeclaration::from(AspectRatio).value("16 / 9");
///
/// assert_eq!(css_aspect_ratio.bake(), "aspect-ratio: 16 / 9;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AspectRatio;

impl DeclarationRecipe for AspectRatio {
    fn property_recipe() -> Bake {
        "aspect-ratio".into()
    }
}
