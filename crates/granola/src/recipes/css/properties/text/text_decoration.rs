use crate::prelude::*;

/// The recipe for the CSS `text-decoration` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration = CssDeclaration::from(TextDecoration).content("none");
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextDecoration;

impl DeclarationRecipe for TextDecoration {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "text-decoration".into()
    }
}
