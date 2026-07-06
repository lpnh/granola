use crate::prelude::*;

/// The recipe for the CSS `appearance` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/appearance)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_appearance = CssDeclaration::from(Appearance).content("button");
///
/// assert_eq!(css_appearance.bake(), "appearance: button;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Appearance;

impl DeclarationRecipe for Appearance {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "appearance".into()
    }
}
