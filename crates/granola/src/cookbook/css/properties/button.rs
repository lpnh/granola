use std::borrow::Cow;

use crate::prelude::*;

/// The `button` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_appearance: CssAppearance<Button> = CssAppearance::from_recipe();
///
/// assert_eq!(css_appearance.bake(), "appearance: button;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Button;

impl AppearanceRecipe for Button {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "button".into();
    }
}
