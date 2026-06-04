use std::borrow::Cow;

use crate::prelude::*;

/// The `::placeholder` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selector = CssSimpleSelector::from(UniversalPlaceholder);
///
/// assert_eq!(selector.bake(), "::placeholder");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalPlaceholder;

impl SimpleSelectorRecipe for UniversalPlaceholder {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::placeholder".into();
    }
}
