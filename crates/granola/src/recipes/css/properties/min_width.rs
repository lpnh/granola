use crate::prelude::*;

/// The recipe for the CSS `min-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/min-width)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_min_width = CssDeclaration::from(MinWidth).value("0");
///
/// assert_eq!(css_min_width.bake(), "min-width: 0;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MinWidth;

impl DeclarationRecipe for MinWidth {
    fn property_recipe() -> Bake {
        "min-width".into()
    }
}
