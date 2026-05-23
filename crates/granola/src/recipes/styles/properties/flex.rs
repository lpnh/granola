use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `flex` property value.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display: CssDisplay<Flex> = CssDisplay::from_recipe();
///
/// assert_eq!(css_display.bake(),
/// "display: flex;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display: CssDisplay<(Inline, Flex)> = CssDisplay::from_recipe();
///
/// assert_eq!(css_display.bake(),
/// "display: inline flex;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Flex;

impl DisplayTag for Flex {
    fn value_recipe(value: &mut Cow<'static, str>) {
        if value.is_empty() {
            *value = "flex".into();
        } else {
            *value = format!("{} {}", value, "flex").into();
        }
    }
}
