use std::borrow::Cow;

use crate::prelude::*;

/// The `dotted` property value recipe.
///
/// Composes with a preceding value, e.g. `underline dotted`.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_text_decoration: CssTextDecoration<(Underline, Dotted)> =
///     CssTextDecoration::from_cookbook();
///
/// assert_eq!(
///     css_text_decoration.bake(),
///     "text-decoration: underline dotted;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dotted;

impl TextDecorationRecipe for Dotted {
    fn value_recipe(value: &mut Cow<'static, str>) {
        if value.is_empty() {
            *value = "dotted".into();
        } else {
            *value = format!("{value} dotted").into();
        }
    }
}

impl WebkitTextDecorationRecipe for Dotted {
    fn value_recipe(value: &mut Cow<'static, str>) {
        if value.is_empty() {
            *value = "dotted".into();
        } else {
            *value = format!("{value} dotted").into();
        }
    }
}
