use crate::prelude::*;

/// The recipe for the CSS `text-underline-offset` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-underline-offset)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_underline_offset = CssDeclaration::from(TextUnderlineOffset).content("0.15em");
///
/// assert_eq!(
///     css_text_underline_offset.bake(),
///     "text-underline-offset: 0.15em;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextUnderlineOffset;

impl DeclarationRecipe for TextUnderlineOffset {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "text-underline-offset".into()
    }
}
