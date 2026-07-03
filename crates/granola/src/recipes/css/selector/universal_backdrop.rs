use crate::prelude::*;

/// The `::backdrop` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selector = CssSimpleSelector::from(UniversalBackdrop);
///
/// assert_eq!(selector.bake(), "::backdrop");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalBackdrop;

impl SimpleSelectorRecipe for UniversalBackdrop {
    fn selector_recipe() -> Bake {
        "::backdrop".into()
    }
}
