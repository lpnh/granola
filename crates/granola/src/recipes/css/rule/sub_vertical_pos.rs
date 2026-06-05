use crate::prelude::*;

/// The `sub { bottom: -0.25em }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SubVerticalPos);
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

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.declarations = vec![CssBottom::new().content("-0.25em").into()];
    }
}
