use crate::{prelude::*, recipes::*};

/// The `::-webkit-inner-spin-button, ::-webkit-outer-spin-button { height: auto
/// }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SpinButtonHeight);
///
/// assert_eq!(
///     rule.bake(),
///     "::-webkit-inner-spin-button,
/// ::-webkit-outer-spin-button {
///   height: auto;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpinButtonHeight;

impl RuleRecipe for SpinButtonHeight {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = ["::-webkit-inner-spin-button", "::-webkit-outer-spin-button"].into();
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.declarations = vec![CssHeight::from(Auto).into()];
    }
}
