use std::borrow::Cow;

use crate::prelude::*;

/// The `baseline` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_vertical_align: CssVerticalAlign<Baseline> = CssVerticalAlign::from_cookbook();
///
/// assert_eq!(css_vertical_align.bake(), "vertical-align: baseline;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Baseline;

impl VerticalAlignRecipe for Baseline {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "baseline".into();
    }
}
