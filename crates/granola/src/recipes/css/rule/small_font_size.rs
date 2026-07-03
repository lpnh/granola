use crate::prelude::*;

/// The `small { font-size: 80% }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SmallFontSize);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "small {
///   font-size: 80%;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SmallFontSize;

impl RuleRecipe for SmallFontSize {
    fn selectors_list_recipe() -> Bake {
        "small".into()
    }

    fn declarations_block_recipe() -> Bake {
        CssFontSize::new().content("80%").into()
    }
}
