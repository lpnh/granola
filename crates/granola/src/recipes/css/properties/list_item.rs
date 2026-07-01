use crate::prelude::*;

/// The `list-item` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display = CssDisplay::from(ListItem);
///
/// assert_eq!(css_display.bake(), "display: list-item;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListItem;

impl DisplayRecipe for ListItem {
    recipe_boilerplate!(DisplayRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "list-item".into();
    }
}
