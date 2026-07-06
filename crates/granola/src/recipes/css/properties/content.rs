use crate::prelude::*;

/// The recipe for the CSS `content` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/content)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_content = CssDeclaration::from(Content).content(r#""this is new""#);
///
/// assert_eq!(css_content.bake(), r#"content: "this is new";"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Content;

impl DeclarationRecipe for Content {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "content".into()
    }
}
