use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `inherit` property value.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_font_family: CssFontFamily<Inherit> = CssFontFamily::from_recipe();
///
/// assert_eq!(css_font_family.bake(), "font-family: inherit;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_font_size: CssFontSize<Inherit> = CssFontSize::from_recipe();
///
/// assert_eq!(css_font_size.bake(), "font-size: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Inherit;

impl FontFamilyRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        if value.is_empty() {
            *value = "inherit".into();
        } else {
            *value = format!("{} {}", value, "inherit").into();
        }
    }
}

impl FontSizeRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}
