use crate::prelude::*;

/// The `sup { top: -0.5em }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let rule: CssRule<SupVerticalPos> = CssRule::from_recipe();
///
/// assert_eq!(
///     rule.bake(),
///     "sup {
///   top: -0.5em;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SupVerticalPos;

impl RuleRecipe for SupVerticalPos {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = "sup".into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![CssTop::<()>::new("-0.5em").into()];
    }
}
