use granola::{prelude::*, recipes::*};

/// The `summary { display: list-item }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SummaryDisplayListItem);
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
pub struct SummaryDisplayListItem;

impl RuleRecipe for SummaryDisplayListItem {
    fn selectors_list_recipe() -> Bake {
        "summary".into()
    }

    fn declarations_block_recipe() -> Bake {
        CssDeclaration::from(Display).content("list-item").into()
    }
}
