use crate::{cookbook::*, prelude::*};

/// The universal reset `box-sizing: border-box` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let rule: CssRule<BoxSizingReset> = CssRule::from_recipe();
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

    fn properties_list_recipe(properties_list: &mut CssDeclarationsBlock) {
        BoxSizingBorderBox::declarations_recipe(&mut properties_list.declarations);
    }
}
