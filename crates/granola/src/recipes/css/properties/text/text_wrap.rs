use crate::prelude::*;

/// The recipe for the CSS `text-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-wrap)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_wrap = CssDeclaration::from(TextWrap).value("balance");
///
/// assert_eq!(css_text_wrap.bake(), "text-wrap: balance;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextWrap;

impl DeclarationRecipe for TextWrap {
    fn property_recipe() -> Bake {
        "text-wrap".into()
    }
}
