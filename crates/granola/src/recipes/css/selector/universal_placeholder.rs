use std::borrow::Cow;

use crate::prelude::*;

/// The `::placeholder` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let selector: CssSimpleSelector<UniversalPlaceholder> = CssSimpleSelector::from_cookbook();
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
