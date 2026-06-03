use std::borrow::Cow;

use crate::prelude::*;

/// The `::backdrop` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let selector: CssSimpleSelector<UniversalBackdrop> = CssSimpleSelector::from_cookbook();
///
/// assert_eq!(selector.bake(), "::backdrop");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalBackdrop;

impl SimpleSelectorRecipe for UniversalBackdrop {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "::backdrop".into();
    }
}
