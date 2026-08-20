use crate::prelude::*;

/// The recipe for the CSS `left` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/left)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_left = CssDeclaration::from(Left).value("1rem");
///
/// assert_eq!(css_left.bake(), "left: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Left;

impl DeclarationRecipe for Left {
    fn property_recipe() -> Bake {
        "left".into()
    }
}
