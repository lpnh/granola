use crate::prelude::*;

/// The recipe for the CSS `margin-bottom` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-bottom)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_margin_bottom = CssDeclaration::from(MarginBottom).content("1rem");
///
/// assert_eq!(css_margin_bottom.bake(), "margin-bottom: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarginBottom;

impl DeclarationRecipe for MarginBottom {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "margin-bottom".into()
    }
}
