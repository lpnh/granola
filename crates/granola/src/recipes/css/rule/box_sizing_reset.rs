use crate::{prelude::*, recipes::*};

/// The universal reset `box-sizing: border-box` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule: CssRule<BoxSizingReset> = CssRule::from_cookbook();
///
/// assert_eq!(
///     rule.bake(),
///     "*,
/// ::after,
/// ::before {
///   box-sizing: border-box;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BoxSizingReset;

impl RuleRecipe for BoxSizingReset {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        UniversalSelectors::selectors_recipe(&mut selectors_list.selectors);
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.push_mut(CssBoxSizing::from(BorderBox));
    }
}
