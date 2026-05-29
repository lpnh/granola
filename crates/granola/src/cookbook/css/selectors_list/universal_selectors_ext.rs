use crate::{cookbook::*, prelude::*};

/// The [`UniversalReset`] + `::backdrop, ::file-selector-button` selectors list
/// recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
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
            CssSelector::<UniversalBackdrop>::from_recipe().bake_recipe(),
            CssSelector::<UniversalFileSelectorButton>::from_recipe().bake_recipe(),
        ])
    }
}
