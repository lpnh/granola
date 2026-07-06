use crate::prelude::*;

/// The recipe for the CSS `min-height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/min-height)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_min_height = CssDeclaration::from(MinHeight).content("1lh");
///
/// assert_eq!(css_min_height.bake(), "min-height: 1lh;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MinHeight;

impl DeclarationRecipe for MinHeight {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "min-height".into()
    }
}
