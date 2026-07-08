use crate::prelude::*;

/// The recipe for the CSS `text-overflow` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-overflow)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_overflow = CssDeclaration::from(TextOverflow).content("ellipsis");
///
/// assert_eq!(css_text_overflow.bake(), "text-overflow: ellipsis;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextOverflow;

impl DeclarationRecipe for TextOverflow {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "text-overflow".into()
    }
}
