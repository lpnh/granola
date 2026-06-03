use crate::prelude::*;

/// The `isolate` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_isolation: CssIsolation<Isolate> = CssIsolation::from_cookbook();
///
/// assert_eq!(css_isolation.bake(), "isolation: isolate;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Isolate;

impl IsolationRecipe for Isolate {
    fn content_recipe(content: &mut Self::Content) {
        *content = "isolate".into();
    }
}
