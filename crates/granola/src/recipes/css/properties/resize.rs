use crate::prelude::*;

/// The recipe for the CSS `resize` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/resize)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_resize = CssDeclaration::from(Resize).content("vertical");
///
/// assert_eq!(css_resize.bake(), "resize: vertical;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Resize;

impl DeclarationRecipe for Resize {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "resize".into()
    }
}
