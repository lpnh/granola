use std::borrow::Cow;

use crate::prelude::*;

/// The `::before` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let selector: CssSimpleSelector<UniversalBefore> = CssSimpleSelector::from_cookbook();
///
/// assert_eq!(selector.bake(), "::before");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalBefore;

impl SimpleSelectorRecipe for UniversalBefore {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::before".into();
    }
}
