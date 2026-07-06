use crate::prelude::*;

/// The recipe for the CSS `margin-top` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-top)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_margin_top = CssDeclaration::from(MarginTop).content("1rem");
///
/// assert_eq!(css_margin_top.bake(), "margin-top: 1rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarginTop;

impl DeclarationRecipe for MarginTop {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "margin-top".into()
    }
}
