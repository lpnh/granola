use crate::{cookbook::*, prelude::*};

/// Rule: `*, *::before, *::after { box-sizing: border-box; }`.
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
///     "*, *::before, *::after {
///   box-sizing: border-box;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BoxSizingReset;

impl RuleRecipe for BoxSizingReset {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        UniversalReset::selectors_recipe(&mut selectors_list.selectors);
    }

    fn properties_list_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![CssBoxSizing::<BorderBox>::from_recipe().into()];
    }
}
