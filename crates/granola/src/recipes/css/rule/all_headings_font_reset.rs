use crate::{prelude::*, recipes::*};

/// The `h1, h2, h3, h4, h5, h6` font reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(AllHeadingsFontReset);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "h1, h2, h3, h4, h5, h6 {
///   font-size: inherit;
///   font-weight: inherit;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllHeadingsFontReset;

impl RuleRecipe for AllHeadingsFontReset {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        AllHeadings::selectors_recipe(&mut selectors_list.selectors);
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssFontSize::from(Inherit).into(),
            CssFontWeight::from(Inherit).into(),
        ]);
    }
}
