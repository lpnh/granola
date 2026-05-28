use std::borrow::Cow;

use crate::prelude::*;

/// The `inline` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_display: CssDisplay<Inline> = CssDisplay::from_recipe();
///
/// assert_eq!(css_display.bake(), "display: inline;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Inline;

impl DisplayRecipe for Inline {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inline".into();
    }
}
