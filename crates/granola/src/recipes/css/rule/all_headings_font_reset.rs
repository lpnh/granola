use crate::{recipes::*, prelude::*};

/// The `h1, h2, h3, h4, h5, h6` font reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let rule: CssRule<AllHeadingsFontReset> = CssRule::from_cookbook();
///
/// assert_eq!(
///     rule.bake(),
///     "h1,
/// h2,
/// h3,
/// h4,
/// h5,
/// h6 {
///   font-size: inherit;
///   font-weight: inherit;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllHeadingsFontReset;

impl RuleRecipe for AllHeadingsFontReset {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSelectorsList::<AllHeadings>::from_cookbook().bake_recipe();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            CssFontSize::<Inherit>::from_cookbook().into(),
            CssFontWeight::<Inherit>::from_cookbook().into(),
        ];
    }
}
