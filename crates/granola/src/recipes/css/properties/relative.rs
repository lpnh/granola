use std::borrow::Cow;

use crate::prelude::*;

/// The `relative` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_position: CssPosition<Relative> = CssPosition::from_cookbook();
///
/// assert_eq!(css_position.bake(), "position: relative;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Relative;

impl PositionRecipe for Relative {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "relative".into();
    }
}
