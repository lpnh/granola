use crate::prelude::*;

/// The `column` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_flex_direction = CssFlexDirection::from(Column);
///
/// assert_eq!(css_flex_direction.bake(), "flex-direction: column;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Column;

impl FlexDirectionRecipe for Column {
    fn content_recipe(content: &mut Self::Content) {
        *content = "column".into();
    }
}
