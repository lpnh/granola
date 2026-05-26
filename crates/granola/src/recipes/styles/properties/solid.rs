use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `solid` property value.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_outline_style: CssOutlineStyle<Solid> = CssOutlineStyle::from_recipe();
///
/// assert_eq!(css_outline_style.bake(), "outline-style: solid;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Solid;

impl OutlineStyleTag for Solid {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "solid".into();
    }
}
