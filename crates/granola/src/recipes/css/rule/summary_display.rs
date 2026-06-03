use crate::{recipes::*, prelude::*};

/// The `summary { display: list-item }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let rule: CssRule<SummaryDisplay> = CssRule::from_cookbook();
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
        *selectors_list = "summary".into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![CssDisplay::<ListItem>::from_cookbook().into()];
    }
}
