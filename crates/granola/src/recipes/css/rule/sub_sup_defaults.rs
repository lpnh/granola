use crate::{recipes::*, prelude::*};

/// The `sub, sup` defaults rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let rule: CssRule<SubSupDefaults> = CssRule::from_recipe();
///
/// assert_eq!(
///     rule.bake(),
///     "sub,
/// sup {
///   font-size: 75%;
///   line-height: 0;
///   position: relative;
///   vertical-align: baseline;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubSupDefaults;

impl RuleRecipe for SubSupDefaults {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = ["sub", "sup"].into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            CssFontSize::<()>::new("75%").into(),
            CssLineHeight::<()>::new("0").into(),
            CssPosition::<Relative>::from_recipe().into(),
            CssVerticalAlign::<Baseline>::from_recipe().into(),
        ];
    }
}
