use std::borrow::Cow;

use crate::prelude::*;

/// The `solid` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_border: CssBorder<Solid> = CssBorder::new("0");
///
/// assert_eq!(css_border.bake(), "border: 0 solid;");
/// ```
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_outline_style: CssOutlineStyle<Solid> = CssOutlineStyle::from_cookbook();
///
/// assert_eq!(css_outline_style.bake(), "outline-style: solid;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Solid;

impl BorderRecipe for Solid {
    fn content_recipe(value: &mut Cow<'static, str>) {
        if value.is_empty() {
            *value = "solid".into();
        } else {
            *value = format!("{} {}", value, "solid").into();
        }
    }
}

impl OutlineStyleRecipe for Solid {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "solid".into();
    }
}
