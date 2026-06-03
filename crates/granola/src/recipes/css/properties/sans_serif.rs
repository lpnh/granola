use std::borrow::Cow;

use crate::prelude::*;

/// The `sans-serif` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_font_family: CssFontFamily<SansSerif> = CssFontFamily::from_cookbook();
///
/// assert_eq!(css_font_family.bake(), "font-family: sans-serif;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SansSerif;

impl FontFamilyRecipe for SansSerif {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "sans-serif".into();
    }
}
