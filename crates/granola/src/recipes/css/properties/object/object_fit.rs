use crate::prelude::*;

/// The recipe for the CSS `object-fit` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/object-fit)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_object_fit = CssDeclaration::from(ObjectFit).content("cover");
///
/// assert_eq!(css_object_fit.bake(), "object-fit: cover;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ObjectFit;

impl DeclarationRecipe for ObjectFit {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "object-fit".into()
    }
}
