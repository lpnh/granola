use crate::prelude::*;

/// The recipe for the CSS `scroll-behavior` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/scroll-behavior)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_scroll_behavior = CssDeclaration::from(ScrollBehavior).content("smooth");
///
/// assert_eq!(css_scroll_behavior.bake(), "scroll-behavior: smooth;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScrollBehavior;

impl DeclarationRecipe for ScrollBehavior {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "scroll-behavior".into()
    }
}
