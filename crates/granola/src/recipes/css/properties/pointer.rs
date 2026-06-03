use crate::prelude::*;

/// The `pointer` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_cursor: CssCursor<Pointer> = CssCursor::from_cookbook();
///
/// assert_eq!(css_cursor.bake(), "cursor: pointer;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pointer;

impl CursorRecipe for Pointer {
    fn content_recipe(content: &mut Self::Content) {
        *content = "pointer".into();
    }
}
