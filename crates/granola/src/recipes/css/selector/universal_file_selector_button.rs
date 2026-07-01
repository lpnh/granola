use crate::prelude::*;

/// The `::file-selector-button` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selector = CssSimpleSelector::from(UniversalFileSelectorButton);
///
/// assert_eq!(selector.bake(), "::file-selector-button");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalFileSelectorButton;

impl SimpleSelectorRecipe for UniversalFileSelectorButton {
    fn selector_recipe(selector: &mut Bake) {
        *selector = "::file-selector-button".into();
    }
}
