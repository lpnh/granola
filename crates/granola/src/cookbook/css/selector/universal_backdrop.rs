use std::borrow::Cow;

use crate::prelude::*;

/// The `::backdrop` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<UniversalBackdrop> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "::backdrop");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalBackdrop;

impl SelectorRecipe for UniversalBackdrop {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::backdrop".into();
    }
}
