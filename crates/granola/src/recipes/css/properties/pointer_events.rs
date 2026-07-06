use crate::prelude::*;

/// The recipe for the CSS `pointer-events` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/pointer-events)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_pointer_events = CssDeclaration::from(PointerEvents).content("none");
///
/// assert_eq!(css_pointer_events.bake(), "pointer-events: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PointerEvents;

impl DeclarationRecipe for PointerEvents {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "pointer-events".into()
    }
}
