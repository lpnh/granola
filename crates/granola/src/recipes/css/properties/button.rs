use crate::prelude::*;

/// The `button` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_appearance = CssAppearance::from(Button);
///
/// assert_eq!(css_appearance.bake(), "appearance: button;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Button;

impl AppearanceRecipe for Button {
    fn content_recipe(content: &mut Self::Content) {
        *content = "button".into();
    }
}
