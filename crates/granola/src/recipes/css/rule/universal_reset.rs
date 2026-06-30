use crate::{prelude::*, recipes::*};

/// The universal reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(UniversalReset);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "*, ::after, ::before, ::backdrop, ::file-selector-button {
///   box-sizing: border-box;
///   margin: 0;
///   padding: 0;
///   border: 0 solid;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalReset;

impl RuleRecipe for UniversalReset {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        UniversalSelectorsExt::selectors_recipe(&mut selectors_list.selectors);
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssBoxSizing::from(BorderBox).into(),
            CssMargin::new().content("0").into(),
            CssPadding::new().content("0").into(),
            CssBorder::new().fold_in("0").fold_in("solid").into(),
        ]);
    }
}
