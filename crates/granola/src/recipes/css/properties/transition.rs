// mod transition_behavior;
// mod transition_delay;
// mod transition_duration;
// mod transition_property;
// mod transition_timing_function;

use crate::prelude::*;

/// The recipe for the CSS `transition` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/transition)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_transition = CssDeclaration::from(Transition).content("background-color 150ms ease");
///
/// assert_eq!(
///     css_transition.bake(),
///     "transition: background-color 150ms ease;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transition;

impl DeclarationRecipe for Transition {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "transition".into()
    }
}
