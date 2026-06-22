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
///     rule.bake_pretty(),
///     "hr {
///   height: 0;
///   color: inherit;
///   border-top-width: 1px;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct HrReset;

impl RuleRecipe for HrReset {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut("hr");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssHeight::new().content("0").into(),
            CssColor::from(Inherit).into(),
            CssBorderTopWidth::new().content("1px").into(),
        ]);
    }
}
