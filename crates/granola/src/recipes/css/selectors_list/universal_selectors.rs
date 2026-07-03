use crate::{prelude::*, recipes::*};

/// The `*, ::after, ::before` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let list = CssSelectorsList::from(UniversalSelectors);
///
/// assert_eq!(list.bake(), "*, ::after, ::before");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalSelectors;

impl SelectorsListRecipe for UniversalSelectors {
    fn selectors_recipe() -> Bake {
        let mut selectors = bake![CssTypeSelector::from(Universal)];
        selectors.fold_in_with(", ", CssSimpleSelector::from(UniversalAfter));
        selectors.fold_in_with(", ", CssSimpleSelector::from(UniversalBefore));
        selectors
    }
}
