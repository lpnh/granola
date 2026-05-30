use crate::{cookbook::*, prelude::*};

/// The `::-webkit-inner-spin-button, ::-webkit-outer-spin-button { height: auto
/// }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let rule: CssRule<SpinButtonHeight> = CssRule::from_recipe();
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

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![CssHeight::<Auto>::from_recipe().into()];
    }
}
