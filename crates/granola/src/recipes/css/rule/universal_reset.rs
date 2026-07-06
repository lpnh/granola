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
    fn selectors_list_recipe() -> Bake {
        UniversalSelectorsExt::selectors_list_recipe()
    }

    fn declarations_block_recipe() -> Bake {
        bake_ws![
            CssBoxSizing::from(BorderBox),
            CssMargin::new().content("0"),
            CssPadding::new().content("0"),
            CssBorder::new().fold_in("0").fold_in("solid"),
        ]
    }
}
