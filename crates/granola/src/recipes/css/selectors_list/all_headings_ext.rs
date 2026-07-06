use crate::{prelude::*, recipes::*};

/// The [`AllHeadings`] + `p` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(AllHeadingsExt);
///
/// assert_eq!(rule.selectors_list, "p, h1, h2, h3, h4, h5, h6");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllHeadingsExt;

impl RuleRecipe for AllHeadingsExt {
    fn selectors_list_recipe() -> Bake {
        bake_comma!["p", AllHeadings::selectors_list_recipe()]
    }
}
