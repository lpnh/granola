use std::borrow::Cow;

use crate::prelude::*;

/// The `::before` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<UniversalBefore> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "::before");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalBefore;

impl SelectorRecipe for UniversalBefore {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::before".into();
    }
}
