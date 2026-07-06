use crate::prelude::*;

/// The recipe for the CSS `white-space` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/white-space)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_white_space = CssDeclaration::from(WhiteSpace).content("nowrap");
///
/// assert_eq!(css_white_space.bake(), "white-space: nowrap;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WhiteSpace;

impl DeclarationRecipe for WhiteSpace {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "white-space".into()
    }
}
