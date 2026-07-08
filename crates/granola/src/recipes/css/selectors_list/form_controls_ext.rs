use crate::prelude::*;

/// The `button, input, optgroup, select, textarea` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(FormControlsExt);
///
/// assert_eq!(
///     rule.selectors_list,
///     "button, input, optgroup, select, textarea"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FormControlsExt;

impl RuleRecipe for FormControlsExt {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma!["button", "input", "optgroup", "select", "textarea"]
    }
}
