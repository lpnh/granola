use crate::prelude::*;

/// The recipe for the CSS `letter-spacing` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/letter-spacing)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_letter_spacing = CssDeclaration::from(LetterSpacing).inherit();
///
/// assert_eq!(css_letter_spacing.bake(), "letter-spacing: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LetterSpacing;

impl DeclarationRecipe for LetterSpacing {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "letter-spacing".into()
    }
}
