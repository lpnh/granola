use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `none` property value.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border: CssBorder<None> = CssBorder::from_recipe();
///
/// assert_eq!(css_border.bake(), "border: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration: CssTextDecoration<None> = CssTextDecoration::from_recipe();
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct None;

impl BorderTag for None {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "none".into();
    }
}

impl TextDecorationTag for None {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "none".into();
    }
}
