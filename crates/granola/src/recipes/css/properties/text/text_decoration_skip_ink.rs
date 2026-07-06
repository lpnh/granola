use crate::prelude::*;

/// The recipe for the CSS `text-decoration-skip-ink` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration-skip-ink)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration_skip_ink = CssDeclaration::from(TextDecorationSkipInk).content("auto");
///
/// assert_eq!(
///     css_text_decoration_skip_ink.bake(),
///     "text-decoration-skip-ink: auto;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextDecorationSkipInk;

impl DeclarationRecipe for TextDecorationSkipInk {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "text-decoration-skip-ink".into()
    }
}
