use crate::prelude::*;

/// The `small { font-size: 80% }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule: CssRule<SmallFontSize> = CssRule::from_cookbook();
///
/// assert_eq!(
///     rule.bake(),
///     "small {
///   font-size: 80%;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SmallFontSize;

impl RuleRecipe for SmallFontSize {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = "small".into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![CssFontSize::new().content("80%").into()];
    }
}
