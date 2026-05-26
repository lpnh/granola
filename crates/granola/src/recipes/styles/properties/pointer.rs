use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `pointer` property value.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_cursor: CssCursor<Pointer> = CssCursor::from_recipe();
///
/// assert_eq!(css_cursor.bake(), "cursor: pointer;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Pointer;

impl CursorTag for Pointer {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "pointer".into();
    }
}
