use crate::{prelude::*, recipes::*};

/// The `hr` reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(HrReset);
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
            CssHeight::new().content("0").into(),
            CssColor::from(Inherit).into(),
            ("border-top-width", "1px").into(),
        ];
    }
}
