use crate::prelude::*;

/// The recipe for the CSS `bottom` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/bottom)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_bottom = CssDeclaration::from(Bottom).content("-0.25em");
///
/// assert_eq!(css_bottom.bake(), "bottom: -0.25em;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bottom;

impl DeclarationRecipe for Bottom {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "bottom".into()
    }
}
