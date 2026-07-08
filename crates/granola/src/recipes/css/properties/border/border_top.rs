use crate::prelude::*;

/// The recipe for the CSS `border-top` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-top)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_top = CssDeclaration::from(BorderTop).content("1px solid black");
///
/// assert_eq!(css_border_top.bake(), "border-top: 1px solid black;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderTop;

impl DeclarationRecipe for BorderTop {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border-top".into()
    }
}
