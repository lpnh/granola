use crate::prelude::*;

/// The recipe for the CSS `border-radius` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-radius)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_radius = CssDeclaration::from(BorderRadius).value("0.5em");
///
/// assert_eq!(css_border_radius.bake(), "border-radius: 0.5em;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderRadius;

impl DeclarationRecipe for BorderRadius {
    fn property_recipe() -> Bake {
        "border-radius".into()
    }
}
