use crate::prelude::*;

/// The `currentcolor` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_color: CssBorderColor<Currentcolor> = CssBorderColor::from_cookbook();
///
/// assert_eq!(css_border_color.bake(), "border-color: currentcolor;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_color: CssColor<Currentcolor> = CssColor::from_cookbook();
///
/// assert_eq!(css_color.bake(), "color: currentcolor;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Currentcolor;

impl BorderColorRecipe for Currentcolor {
    fn content_recipe(content: &mut Self::Content) {
        *content = "currentcolor".into();
    }
}

impl ColorRecipe for Currentcolor {
    fn content_recipe(content: &mut Self::Content) {
        *content = "currentcolor".into();
    }
}
