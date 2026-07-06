use crate::prelude::*;

/// The recipe for the CSS `position-try-fallbacks` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position-try-fallbacks)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_position_try_fallbacks =
///     CssDeclaration::from(PositionTryFallbacks).content("flip-block");
///
/// assert_eq!(
///     css_position_try_fallbacks.bake(),
///     "position-try-fallbacks: flip-block;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PositionTryFallbacks;

impl DeclarationRecipe for PositionTryFallbacks {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "position-try-fallbacks".into()
    }
}
