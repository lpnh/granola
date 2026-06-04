use std::borrow::Cow;

use crate::prelude::*;

/// The `::after` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selector = CssSimpleSelector::from(UniversalAfter);
///
/// assert_eq!(selector.bake(), "::after");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalAfter;

impl SimpleSelectorRecipe for UniversalAfter {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::after".into();
    }
}
