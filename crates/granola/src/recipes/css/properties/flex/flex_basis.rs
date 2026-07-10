use crate::prelude::*;

/// The recipe for the CSS `flex-basis` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex-basis)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_flex_basis = CssDeclaration::from(FlexBasis).content("100%");
///
/// assert_eq!(css_flex_basis.bake(), "flex-basis: 100%;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FlexBasis;

impl DeclarationRecipe for FlexBasis {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "flex-basis".into()
    }
}
