use std::borrow::Cow;

use crate::prelude::*;

/// The `underline` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_text_decoration: CssTextDecoration<Underline> = CssTextDecoration::from_recipe();
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: underline;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Underline;

impl TextDecorationRecipe for Underline {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "underline".into();
    }
}

impl WebkitTextDecorationRecipe for Underline {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "underline".into();
    }
}
