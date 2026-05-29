use crate::{cookbook::*, prelude::*};

/// The universal reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let rule: CssRule<UniversalReset> = CssRule::from_recipe();
///
/// assert_eq!(
///     rule.bake(),
///     "*,
/// ::after,
/// ::before,
/// ::backdrop,
/// ::file-selector-button {
///   box-sizing: border-box;
///   margin: 0;
///   padding: 0;
///   border: 0 solid;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalReset;

impl RuleRecipe for UniversalReset {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        UniversalSelectorsExt::selectors_recipe(&mut selectors_list.selectors);
    }

    fn properties_list_recipe(properties_list: &mut CssDeclarationsBlock) {
        BoxSizingBorderBox::declarations_recipe(&mut properties_list.declarations);
        MarginPaddingReset::declarations_recipe(&mut properties_list.declarations);
        BorderReset::declarations_recipe(&mut properties_list.declarations);
    }
}
