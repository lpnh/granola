use crate::prelude::*;

/// The recipe for the CSS `background-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/background-color)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_background_color = CssDeclaration::from(BackgroundColor).content("transparent");
///
/// assert_eq!(
///     css_background_color.bake(),
///     "background-color: transparent;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BackgroundColor;

impl DeclarationRecipe for BackgroundColor {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "background-color".into()
    }
}
