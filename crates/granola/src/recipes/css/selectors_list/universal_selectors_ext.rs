use crate::{prelude::*, recipes::*};

/// The [`UniversalReset`] + `::backdrop, ::file-selector-button` selectors list
/// recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(UniversalSelectorsExt);
///
/// assert_eq!(
///     rule.selectors_list,
///     "*, ::after, ::before, ::backdrop, ::file-selector-button"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalSelectorsExt;

impl RuleRecipe for UniversalSelectorsExt {
    fn selectors_list_recipe() -> Bake {
        bake_comma![
            UniversalSelectors::selectors_list_recipe(),
            CssSimpleSelector::from(UniversalBackdrop),
            CssSimpleSelector::from(UniversalFileSelectorButton),
        ]
    }
}
