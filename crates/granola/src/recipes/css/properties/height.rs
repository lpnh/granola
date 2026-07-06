use crate::prelude::*;

/// The recipe for the CSS `height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/height)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_height = CssDeclaration::from(Height).content("auto");
///
/// assert_eq!(css_height.bake(), "height: auto;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Height;

impl DeclarationRecipe for Height {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "height".into()
    }
}
