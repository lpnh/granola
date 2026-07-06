use crate::prelude::*;

/// The recipe for the CSS `width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/width)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_width = CssDeclaration::from(Width).content("100%");
///
/// assert_eq!(css_width.bake(), "width: 100%;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Width;

impl DeclarationRecipe for Width {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "width".into()
    }
}
