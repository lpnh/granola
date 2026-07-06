use crate::prelude::*;

/// The recipe for the CSS `margin-right` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-right)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_margin_right = CssDeclaration::from(MarginRight).content("1rem");
///
/// assert_eq!(css_margin_right.bake(), "margin-right: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarginRight;

impl DeclarationRecipe for MarginRight {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "margin-right".into()
    }
}
