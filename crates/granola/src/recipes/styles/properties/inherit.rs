use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `inherit` property value.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_family: CssFontFamily<Inherit> = CssFontFamily::from_recipe();
///
/// assert_eq!(css_font_family.bake(), "font-family: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Inherit;

impl FontFamilyTag for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        if value.is_empty() {
            *value = "inherit".into();
        } else {
            *value = format!("{} {}", value, "inherit").into();
        }
    }
}
