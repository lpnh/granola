use crate::{prelude::*, recipes::*};

/// The `h1, h2, h3, h4, h5, h6` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selectors_list = CssSelectorsList::from(AllHeadings);
///
/// assert_eq!(selectors_list.bake(), "h1, h2, h3, h4, h5, h6");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllHeadings;

impl SelectorsListRecipe for AllHeadings {
    fn selectors_recipe() -> Bake {
        let mut selectors = Headings::selectors_recipe();
        selectors.fold_in_with(", ", "h5");
        selectors.fold_in_with(", ", "h6");
        selectors
    }
}
