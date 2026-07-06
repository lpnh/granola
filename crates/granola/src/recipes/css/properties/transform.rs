// mod transform_box;
// mod transform_origin;
// mod transform_style;

use crate::prelude::*;

/// The recipe for the CSS `transform` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/transform)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_transform = CssDeclaration::from(Transform).content("scale(0.97)");
///
/// assert_eq!(css_transform.bake(), "transform: scale(0.97);");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transform;

impl DeclarationRecipe for Transform {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "transform".into()
    }
}
