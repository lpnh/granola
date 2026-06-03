use std::borrow::Cow;

use crate::prelude::*;

/// The `pointer` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_cursor: CssCursor<Pointer> = CssCursor::from_cookbook();
///
/// assert_eq!(css_cursor.bake(), "cursor: pointer;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pointer;

impl CursorRecipe for Pointer {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "pointer".into();
    }
}
