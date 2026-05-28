use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `currentColor` property value.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_color: CssColor<Currentcolor> = CssColor::from_recipe();
///
/// assert_eq!(css_color.bake(), "color: currentcolor;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Currentcolor;

impl ColorRecipe for Currentcolor {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "currentcolor".into();
    }
}
