use crate::{recipes::*, prelude::*};

/// The [`UniversalReset`] + `::backdrop, ::file-selector-button` selectors list
/// recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let list: CssSelectorsList<UniversalSelectorsExt> = CssSelectorsList::from_recipe();
///
/// assert_eq!(
///     list.bake(),
///     "*,
/// ::after,
/// ::before,
/// ::backdrop,
/// ::file-selector-button"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalSelectorsExt;

impl SelectorsListRecipe for UniversalSelectorsExt {
    fn selectors_recipe(selectors: &mut Vec<CssSelector>) {
        UniversalSelectors::selectors_recipe(selectors);
        selectors.extend([
            CssSimpleSelector::<UniversalBackdrop>::from_recipe().into(),
            CssSimpleSelector::<UniversalFileSelectorButton>::from_recipe().into(),
        ])
    }
}
