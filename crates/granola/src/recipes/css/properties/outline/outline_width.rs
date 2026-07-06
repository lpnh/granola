use crate::prelude::*;

/// The recipe for the CSS `outline-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-width)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_outline_width = CssDeclaration::from(OutlineWidth).content("2px");
///
/// assert_eq!(css_outline_width.bake(), "outline-width: 2px;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OutlineWidth;

impl DeclarationRecipe for OutlineWidth {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "outline-width".into()
    }
}
