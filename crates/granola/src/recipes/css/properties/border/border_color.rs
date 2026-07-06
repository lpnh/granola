use crate::prelude::*;

/// The recipe for the CSS `border-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-color)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_color = CssDeclaration::from(BorderColor).content("currentcolor");
///
/// assert_eq!(css_border_color.bake(), "border-color: currentcolor;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderColor;

impl DeclarationRecipe for BorderColor {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border-color".into()
    }
}
