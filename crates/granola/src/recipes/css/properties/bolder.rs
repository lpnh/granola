use std::borrow::Cow;

use crate::prelude::*;

/// The `bolder` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_font_weight: CssFontWeight<Bolder> = CssFontWeight::from_cookbook();
///
/// assert_eq!(css_font_weight.bake(), "font-weight: bolder;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bolder;

impl FontWeightRecipe for Bolder {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "bolder".into();
    }
}
