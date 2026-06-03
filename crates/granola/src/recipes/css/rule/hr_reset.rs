use crate::{recipes::*, prelude::*};

/// The `hr` reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let rule: CssRule<HrReset> = CssRule::from_recipe();
///
/// assert_eq!(
///     rule.bake(),
///     "hr {
///   height: 0;
///   color: inherit;
///   border-top-width: 1px;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HrReset;

impl RuleRecipe for HrReset {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = "hr".into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            CssHeight::<()>::new("0").into(),
            CssColor::<Inherit>::from_recipe().into(),
            ("border-top-width", "1px").into(),
        ];
    }
}
