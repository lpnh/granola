use crate::prelude::*;

/// The `sub { bottom: -0.25em }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let rule: CssRule<SubVerticalPos> = CssRule::from_recipe();
///
/// assert_eq!(
///     rule.bake(),
///     "sub {
///   bottom: -0.25em;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubVerticalPos;

impl RuleRecipe for SubVerticalPos {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = "sub".into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![CssBottom::<()>::new("-0.25em").into()];
    }
}
