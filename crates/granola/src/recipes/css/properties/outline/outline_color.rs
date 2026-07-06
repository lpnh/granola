use crate::prelude::*;

/// The recipe for the CSS `outline-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-color)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_outline_color = CssDeclaration::from(OutlineColor).inherit();
///
/// assert_eq!(css_outline_color.bake(), "outline-color: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OutlineColor;

impl DeclarationRecipe for OutlineColor {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "outline-color".into()
    }
}
