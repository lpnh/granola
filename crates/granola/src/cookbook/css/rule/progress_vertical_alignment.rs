use crate::{cookbook::*, prelude::*};

/// The `progress { vertical-align: baseline }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let rule: CssRule<ProgressVerticalAlignment> = CssRule::from_recipe();
///
/// assert_eq!(
///     rule.bake(),
///     "progress {
///   vertical-align: baseline;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ProgressVerticalAlignment;

impl RuleRecipe for ProgressVerticalAlignment {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = "progress".into();
    }

    fn properties_list_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![CssVerticalAlign::<Baseline>::from_recipe().into()];
    }
}
