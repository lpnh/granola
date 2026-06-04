use crate::prelude::*;

/// The `flex` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display = CssDisplay::from(Flex);
///
/// assert_eq!(css_display.bake(), "display: flex;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Flex;

impl DisplayRecipe for Flex {
    fn content_recipe(content: &mut Self::Content) {
        if content.is_empty() {
            *content = "flex".into();
        } else {
            *content = format!("{} {}", content, "flex").into();
        }
    }
}
