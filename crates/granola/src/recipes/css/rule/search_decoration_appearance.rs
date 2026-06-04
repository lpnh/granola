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
///     rule.bake(),
///     "::-webkit-search-decoration {
///   -webkit-appearance: none;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SearchDecorationAppearance;

impl RuleRecipe for SearchDecorationAppearance {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = "::-webkit-search-decoration".into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![("-webkit-appearance", "none").into()];
    }
}
