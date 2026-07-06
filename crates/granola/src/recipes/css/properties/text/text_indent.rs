use crate::prelude::*;

/// The recipe for the CSS `text-indent` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-indent)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_indent = CssDeclaration::from(TextIndent).content("0");
///
/// assert_eq!(css_text_indent.bake(), "text-indent: 0;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextIndent;

impl DeclarationRecipe for TextIndent {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "text-indent".into()
    }
}
