use crate::prelude::*;

/// The `code, kbd, samp, pre` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(MonospaceSelectors);
///
/// assert_eq!(rule.selectors_list, "code, kbd, samp, pre");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MonospaceSelectors;

impl RuleRecipe for MonospaceSelectors {
    fn selectors_list_recipe() -> Bake {
        bake_comma!["code", "kbd", "samp", "pre"]
    }
}
