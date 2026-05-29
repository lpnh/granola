use std::borrow::Cow;

use crate::prelude::*;

/// The `::file-selector-button` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<UniversalFileSelectorButton> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "::file-selector-button");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalFileSelectorButton;

impl SelectorRecipe for UniversalFileSelectorButton {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::file-selector-button".into();
    }
}
