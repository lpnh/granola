use std::borrow::Cow;

use crate::prelude::*;

/// The `flex` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_display: CssDisplay<Flex> = CssDisplay::from_recipe();
///
/// assert_eq!(css_display.bake(), "display: flex;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_display: CssDisplay<(Inline, Flex)> = CssDisplay::from_recipe();
///
/// assert_eq!(css_display.bake(), "display: inline flex;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Flex;

impl DisplayRecipe for Flex {
    fn value_recipe(value: &mut Cow<'static, str>) {
        if value.is_empty() {
            *value = "flex".into();
        } else {
            *value = format!("{} {}", value, "flex").into();
        }
    }
}
