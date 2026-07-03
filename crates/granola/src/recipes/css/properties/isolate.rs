use crate::prelude::*;

/// The `isolate` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_isolation = CssIsolation::from(Isolate);
///
/// assert_eq!(css_isolation.bake(), "isolation: isolate;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Isolate;

impl IsolationRecipe for Isolate {
    recipe_boilerplate!(IsolationRecipe);

    fn content_recipe() -> Self::Content {
        "isolate".into()
    }
}
