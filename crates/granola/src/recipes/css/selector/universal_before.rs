use crate::prelude::*;

/// The `::before` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selector = CssSimpleSelector::from(UniversalBefore);
///
/// assert_eq!(selector.bake(), "::before");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalBefore;

impl SimpleSelectorRecipe for UniversalBefore {
    fn selector_recipe(selector: &mut Bake) {
        *selector = "::before".into();
    }
}
