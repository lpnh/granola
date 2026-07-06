use crate::prelude::*;

/// The recipe for the CSS `isolation` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/isolation)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_isolation = CssDeclaration::from(Isolation).content("isolate");
///
/// assert_eq!(css_isolation.bake(), "isolation: isolate;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Isolation;

impl DeclarationRecipe for Isolation {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "isolation".into()
    }
}
