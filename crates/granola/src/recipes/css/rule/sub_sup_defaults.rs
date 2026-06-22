use crate::{prelude::*, recipes::*};

/// The `sub, sup` defaults rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SubSupDefaults);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "sub, sup {
///   font-size: 75%;
///   line-height: 0;
///   position: relative;
///   vertical-align: baseline;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubSupDefaults;

impl RuleRecipe for SubSupDefaults {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.extend_mut(["sub".into(), "sup".into()]);
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssFontSize::new().content("75%").into(),
            CssLineHeight::new().content("0").into(),
            CssPosition::from(Relative).into(),
            CssVerticalAlign::from(Baseline).into(),
        ]);
    }
}
