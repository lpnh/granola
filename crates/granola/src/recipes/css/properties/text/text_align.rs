use crate::prelude::*;

/// The recipe for the CSS `text-align` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-align)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_align = CssDeclaration::from(TextAlign).inherit();
///
/// assert_eq!(css_text_align.bake(), "text-align: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextAlign;

impl DeclarationRecipe for TextAlign {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "text-align".into()
    }
}
