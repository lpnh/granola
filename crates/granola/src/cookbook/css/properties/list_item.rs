use std::borrow::Cow;

use crate::prelude::*;

/// The `list-item` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_display: CssDisplay<ListItem> = CssDisplay::from_recipe();
///
/// assert_eq!(css_display.bake(), "display: list-item;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListItem;

impl DisplayRecipe for ListItem {
    fn content_recipe(value: &mut Cow<'static, str>) {
        *value = "list-item".into();
    }
}
