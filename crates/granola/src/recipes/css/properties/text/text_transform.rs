use crate::prelude::*;

/// The recipe for the CSS `text-transform` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-transform)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_transform = CssDeclaration::from(TextTransform).content("uppercase");
///
/// assert_eq!(css_text_transform.bake(), "text-transform: uppercase;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextTransform;

impl DeclarationRecipe for TextTransform {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "text-transform".into()
    }
}
