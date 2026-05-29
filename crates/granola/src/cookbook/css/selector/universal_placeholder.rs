use std::borrow::Cow;

use crate::prelude::*;

/// The `::placeholder` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<UniversalPlaceholder> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "::placeholder");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalPlaceholder;

impl SelectorRecipe for UniversalPlaceholder {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::placeholder".into();
    }
}
