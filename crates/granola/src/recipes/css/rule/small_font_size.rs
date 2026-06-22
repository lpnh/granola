use crate::prelude::*;

/// The `small { font-size: 80% }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SmallFontSize);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "small {
///   font-size: 80%;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SmallFontSize;

impl RuleRecipe for SmallFontSize {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut("small");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.push_mut(CssFontSize::new().content("80%"));
    }
}
