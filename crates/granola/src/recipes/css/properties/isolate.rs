use std::borrow::Cow;

use crate::prelude::*;

/// The `isolate` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_isolation: CssIsolation<Isolate> = CssIsolation::from_recipe();
///
/// assert_eq!(css_isolation.bake(), "isolation: isolate;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Isolate;

impl IsolationRecipe for Isolate {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "isolate".into();
    }
}
