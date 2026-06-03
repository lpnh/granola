use crate::{recipes::*, prelude::*};

/// The `b, strong { font-weight: bolder }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let rule: CssRule<BStrongFontWeight> = CssRule::from_cookbook();
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
        *selectors_list = ["b", "strong"].into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![CssFontWeight::<Bolder>::from_cookbook().into()];
    }
}
