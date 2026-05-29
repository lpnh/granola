use std::borrow::Cow;

use crate::prelude::*;

/// The `middle` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_vertical_align: CssVerticalAlign<Middle> = CssVerticalAlign::from_recipe();
///
/// assert_eq!(css_vertical_align.bake(), "vertical-align: middle;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Middle;

impl VerticalAlignRecipe for Middle {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "middle".into();
    }
}
