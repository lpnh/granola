use crate::prelude::*;

/// The `!important` flag value recipe.
///
/// Appends `!important` to whichever value it composes with, so any property
/// can opt in via a tuple recipe such as `(None, Important)`.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display: CssDisplay<(None, Important)> = CssDisplay::from_cookbook();
///
/// assert_eq!(css_display.bake(), "display: none !important;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Important;

impl DisplayRecipe for Important {
    fn content_recipe(content: &mut Self::Content) {
        if content.is_empty() {
            *content = "!important".into();
        } else {
            *content = format!("{content} !important").into();
        }
    }
}
