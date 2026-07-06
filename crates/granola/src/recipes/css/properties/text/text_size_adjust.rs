use crate::prelude::*;

/// The recipe for the CSS `text-size-adjust` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-size-adjust)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_size_adjust = CssDeclaration::from(TextSizeAdjust).content("none");
///
/// assert_eq!(css_text_size_adjust.bake(), "text-size-adjust: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextSizeAdjust;

impl DeclarationRecipe for TextSizeAdjust {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "text-size-adjust".into()
    }
}
