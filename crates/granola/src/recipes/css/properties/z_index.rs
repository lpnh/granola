use crate::prelude::*;

/// The recipe for the CSS `z-index` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/z-index)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_z_index = CssDeclaration::from(ZIndex).content("1");
///
/// assert_eq!(css_z_index.bake(), "z-index: 1;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ZIndex;

impl DeclarationRecipe for ZIndex {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "z-index".into()
    }
}
