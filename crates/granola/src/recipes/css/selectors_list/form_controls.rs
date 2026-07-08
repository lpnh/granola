use crate::prelude::*;

/// The `button, input, select, textarea` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(FormControls);
///
/// assert_eq!(rule.selectors_list, "button, input, select, textarea");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FormControls;

impl RuleRecipe for FormControls {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma!["button", "input", "select", "textarea"]
    }
}
