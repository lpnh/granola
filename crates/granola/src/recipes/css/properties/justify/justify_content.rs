use crate::prelude::*;

/// The recipe for the CSS `justify-content` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/justify-content)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_justify_content = CssDeclaration::from(JustifyContent).content("center");
///
/// assert_eq!(css_justify_content.bake(), "justify-content: center;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct JustifyContent;

impl DeclarationRecipe for JustifyContent {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "justify-content".into()
    }
}
