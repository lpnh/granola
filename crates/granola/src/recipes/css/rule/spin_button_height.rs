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
///     rule.bake_pretty(),
///     "::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
///   height: auto;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpinButtonHeight;

impl RuleRecipe for SpinButtonHeight {
    fn selectors_list_recipe() -> Bake {
        let mut selectors_list = Bake::default();
        for selector in ["::-webkit-inner-spin-button", "::-webkit-outer-spin-button"] {
            selectors_list.fold_in_with(", ", selector);
        }
        selectors_list
    }

    fn declarations_block_recipe() -> Bake {
        CssHeight::from(Auto).into()
    }
}
