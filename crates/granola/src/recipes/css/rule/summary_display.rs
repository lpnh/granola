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
///     rule.bake(),
///     "summary {
///   display: list-item;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SummaryDisplay;

impl RuleRecipe for SummaryDisplay {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut("summary");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.push_mut(CssDisplay::from(ListItem));
    }
}
