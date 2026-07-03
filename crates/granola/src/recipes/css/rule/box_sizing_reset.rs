use crate::{prelude::*, recipes::*};

/// The universal reset `box-sizing: border-box` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(BoxSizingReset);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "*, ::after, ::before {
///   box-sizing: border-box;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BoxSizingReset;

impl RuleRecipe for BoxSizingReset {
    fn selectors_list_recipe() -> Bake {
        UniversalSelectors::selectors_recipe()
    }

    fn declarations_block_recipe() -> Bake {
        CssBoxSizing::from(BorderBox).into()
    }
}
