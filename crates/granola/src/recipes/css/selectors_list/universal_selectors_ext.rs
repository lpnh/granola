use crate::{prelude::*, recipes::*};

/// The [`UniversalReset`] + `::backdrop, ::file-selector-button` selectors list
/// recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let list = CssSelectorsList::from(UniversalSelectorsExt);
///
/// assert_eq!(
///     list.bake(),
///     "*, ::after, ::before, ::backdrop, ::file-selector-button"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalSelectorsExt;

impl SelectorsListRecipe for UniversalSelectorsExt {
    fn selectors_recipe() -> Bake {
        let mut selectors = UniversalSelectors::selectors_recipe();
        selectors.fold_in_with(", ", CssSimpleSelector::from(UniversalBackdrop));
        selectors.fold_in_with(", ", CssSimpleSelector::from(UniversalFileSelectorButton));
        selectors
    }
}
