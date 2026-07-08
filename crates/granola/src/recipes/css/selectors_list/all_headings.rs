use crate::{prelude::*, recipes::*};

/// The `h1, h2, h3, h4, h5, h6` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(AllHeadings);
///
/// assert_eq!(rule.selectors_list, "h1, h2, h3, h4, h5, h6");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllHeadings;

impl RuleRecipe for AllHeadings {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma![Headings::selectors_list_recipe(), "h5", "h6"]
    }
}
