use crate::prelude::*;

/// The recipe for the CSS `margin-left` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-left)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_margin_left = CssDeclaration::from(MarginLeft).content("1rem");
///
/// assert_eq!(css_margin_left.bake(), "margin-left: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarginLeft;

impl DeclarationRecipe for MarginLeft {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "margin-left".into()
    }
}
