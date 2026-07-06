use crate::prelude::*;

/// The recipe for the CSS `display` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/display)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display = CssDeclaration::from(Display).content("inline flex");
///
/// assert_eq!(css_display.bake(), "display: inline flex;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Display;

impl DeclarationRecipe for Display {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "display".into()
    }
}
