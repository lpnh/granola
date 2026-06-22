use crate::prelude::*;

/// The `::-webkit-search-decoration { -webkit-appearance: none }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SearchDecorationAppearance);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "::-webkit-search-decoration {
///   -webkit-appearance: none;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SearchDecorationAppearance;

impl RuleRecipe for SearchDecorationAppearance {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut("::-webkit-search-decoration");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.push_mut(("-webkit-appearance", "none"));
    }
}
