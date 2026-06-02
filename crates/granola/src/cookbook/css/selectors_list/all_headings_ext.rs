use crate::{cookbook::*, prelude::*};

/// The [`AllHeadings`] + `p` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selectors_list: CssSelectorsList<AllHeadingsExt> = CssSelectorsList::from_recipe();
///
/// assert_eq!(
///     selectors_list.bake(),
///     "p,
/// h1,
/// h2,
/// h3,
/// h4,
/// h5,
/// h6"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllHeadingsExt;

impl SelectorsListRecipe for AllHeadingsExt {
    fn selectors_recipe(selectors: &mut Vec<CssSelector>) {
        selectors.push("p".into());
        AllHeadings::selectors_recipe(selectors);
    }
}
