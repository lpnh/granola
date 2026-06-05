use std::borrow::Cow;

use crate::prelude::*;

/// The `*` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selector = CssTypeSelector::from(Universal);
///
/// assert_eq!(selector.bake(), "*");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Universal;

impl TypeSelectorRecipe for Universal {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "*".into();
    }
}
