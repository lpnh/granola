use std::borrow::Cow;

use crate::prelude::*;

/// The `border-box` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_box_sizing: CssBoxSizing<BorderBox> = CssBoxSizing::from_cookbook();
///
/// assert_eq!(css_box_sizing.bake(), "box-sizing: border-box;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderBox;

impl BoxSizingRecipe for BorderBox {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "border-box".into();
    }
}
