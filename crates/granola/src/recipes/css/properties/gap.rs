use crate::prelude::*;

/// The recipe for the CSS `gap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/gap)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_gap = CssDeclaration::from(Gap).content("1rem");
///
/// assert_eq!(css_gap.bake(), "gap: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Gap;

impl DeclarationRecipe for Gap {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "gap".into()
    }
}
