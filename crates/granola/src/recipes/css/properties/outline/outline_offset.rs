use crate::prelude::*;

/// The recipe for the CSS `outline-offset` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-offset)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_outline_offset = CssDeclaration::from(OutlineOffset).content("2px");
///
/// assert_eq!(css_outline_offset.bake(), "outline-offset: 2px;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OutlineOffset;

impl DeclarationRecipe for OutlineOffset {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "outline-offset".into()
    }
}
