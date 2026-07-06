use crate::prelude::*;

/// The recipe for the CSS `top` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/top)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_top = CssDeclaration::from(Top).content("-0.5em");
///
/// assert_eq!(css_top.bake(), "top: -0.5em;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Top;

impl DeclarationRecipe for Top {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "top".into()
    }
}
