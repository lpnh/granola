use crate::prelude::*;

/// The recipe for the CSS `opacity` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/opacity)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_opacity = CssDeclaration::from(Opacity).content("1");
///
/// assert_eq!(css_opacity.bake(), "opacity: 1;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Opacity;

impl DeclarationRecipe for Opacity {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "opacity".into()
    }
}
