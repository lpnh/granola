use crate::{prelude::*, recipes::*};

/// The `summary { display: list-item }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SummaryDisplay);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "summary {
///   display: list-item;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SummaryDisplay;

impl RuleRecipe for SummaryDisplay {
    fn selectors_list_recipe() -> Bake {
        "summary".into()
    }

    fn declarations_block_recipe() -> Bake {
        CssDisplay::from(ListItem).into()
    }
}
