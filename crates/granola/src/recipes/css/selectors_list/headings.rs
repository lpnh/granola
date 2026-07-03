use crate::prelude::*;

/// The `h1, h2, h3, h4` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selectors_list = CssSelectorsList::from(Headings);
///
/// assert_eq!(selectors_list.bake(), "h1, h2, h3, h4");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headings;

impl SelectorsListRecipe for Headings {
    fn selectors_recipe() -> Bake {
        let mut selectors = <Bake>::default();
        selectors.fold_in_with(", ", "h1");
        selectors.fold_in_with(", ", "h2");
        selectors.fold_in_with(", ", "h3");
        selectors.fold_in_with(", ", "h4");
        selectors
    }
}
