use crate::prelude::*;

/// The `h1, h2, h3, h4` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(Headings);
///
/// assert_eq!(rule.selectors_list, "h1, h2, h3, h4");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headings;

impl RuleRecipe for Headings {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma!["h1", "h2", "h3", "h4"]
    }
}
