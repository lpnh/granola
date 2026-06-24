use crate::{prelude::*, recipes::*};

/// The `a:not([class])` defaults rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(AnchorDefaults);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "a:not([class]) {
///   text-decoration-skip-ink: auto;
///   color: currentcolor;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnchorDefaults;

impl RuleRecipe for AnchorDefaults {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut("a:not([class])");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssTextDecorationSkipInk::from(Auto).into(),
            CssColor::from(Currentcolor).into(),
        ]);
    }
}
