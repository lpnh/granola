use std::borrow::Cow;

use crate::prelude::*;

/// The `*` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<Universal> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "*");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Universal;

impl SelectorRecipe for Universal {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "*".into();
    }
}
