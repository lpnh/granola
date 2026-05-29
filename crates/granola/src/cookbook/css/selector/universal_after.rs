use std::borrow::Cow;

use crate::prelude::*;

/// The `::after` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<UniversalAfter> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "::after");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalAfter;

impl SelectorRecipe for UniversalAfter {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::after".into();
    }
}
