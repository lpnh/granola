use std::borrow::Cow;

use crate::prelude::*;

/// The `column` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_flex_direction: CssFlexDirection<Column> = CssFlexDirection::from_cookbook();
///
/// assert_eq!(css_flex_direction.bake(), "flex-direction: column;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Column;

impl FlexDirectionRecipe for Column {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "column".into();
    }
}
