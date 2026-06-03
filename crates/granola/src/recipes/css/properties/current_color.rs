use std::borrow::Cow;

use crate::prelude::*;

/// The `currentcolor` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_border_color: CssBorderColor<Currentcolor> = CssBorderColor::from_cookbook();
///
/// assert_eq!(css_border_color.bake(), "border-color: currentcolor;");
/// ```
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_color: CssColor<Currentcolor> = CssColor::from_cookbook();
///
/// assert_eq!(css_color.bake(), "color: currentcolor;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Currentcolor;

impl BorderColorRecipe for Currentcolor {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "currentcolor".into();
    }
}

impl ColorRecipe for Currentcolor {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "currentcolor".into();
    }
}
