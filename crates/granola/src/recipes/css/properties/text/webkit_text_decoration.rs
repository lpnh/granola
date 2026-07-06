use crate::prelude::*;

/// The recipe for the CSS `-webkit-text-decoration` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css = CssDeclaration::from(WebkitTextDecoration).content("none");
///
/// assert_eq!(css.bake(), "-webkit-text-decoration: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WebkitTextDecoration;

impl DeclarationRecipe for WebkitTextDecoration {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "-webkit-text-decoration".into()
    }
}
