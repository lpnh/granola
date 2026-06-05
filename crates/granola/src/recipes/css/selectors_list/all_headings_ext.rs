use crate::{prelude::*, recipes::*};

/// The [`AllHeadings`] + `p` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selectors_list = CssSelectorsList::from(AllHeadingsExt);
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
    fn selectors_recipe(selectors: &mut Vec<CssComplexSelector>) {
        selectors.push("p".into());
        AllHeadings::selectors_recipe(selectors);
    }
}
