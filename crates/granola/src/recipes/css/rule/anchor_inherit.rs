use crate::{prelude::*, recipes::*};

/// The `a` inherit rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(AnchorInherit);
///
/// assert_eq!(
///     rule.bake(),
///     "a {
///   color: inherit;
///   -webkit-text-decoration: inherit;
///   text-decoration: inherit;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnchorInherit;

impl RuleRecipe for AnchorInherit {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut("a");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssColor::from(Inherit).into(),
            CssWebkitTextDecoration::from(Inherit).into(),
            CssTextDecoration::from(Inherit).into(),
        ]);
    }
}
