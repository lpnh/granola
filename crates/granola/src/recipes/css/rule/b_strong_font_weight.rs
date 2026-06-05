use crate::{prelude::*, recipes::*};

/// The `b, strong { font-weight: bolder }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(BStrongFontWeight);
///
/// assert_eq!(
///     rule.bake(),
///     "b,
/// strong {
///   font-weight: bolder;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BStrongFontWeight;

impl RuleRecipe for BStrongFontWeight {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.extend_mut(["b".into(), "strong".into()]);
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.push_mut(CssFontWeight::from(Bolder));
    }
}
