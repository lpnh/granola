use crate::{cookbook::*, prelude::*};

/// The `h1, h2, h3, h4, h5, h6` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selectors_list: CssSelectorsList<AllHeadings> = CssSelectorsList::from_recipe();
///
/// assert_eq!(
///     selectors_list.bake(),
///     "h1,
/// h2,
/// h3,
/// h4,
/// h5,
/// h6"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllHeadings;

impl SelectorsListRecipe for AllHeadings {
    fn selectors_recipe(selectors: &mut Vec<CssSelector>) {
        Headings::selectors_recipe(selectors);
        selectors.extend(["h5".into(), "h6".into()]);
    }
}
