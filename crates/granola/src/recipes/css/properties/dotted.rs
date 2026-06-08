use crate::prelude::*;

/// The `dotted` property value recipe.
///
/// Composes with a preceding value, e.g. `underline dotted`.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
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
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        if content.is_empty() {
            *content = "dotted".into();
        } else {
            *content = format!("{content} dotted").into();
        }
    }
}

impl WebkitTextDecorationRecipe for Dotted {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        if content.is_empty() {
            *content = "dotted".into();
        } else {
            *content = format!("{content} dotted").into();
        }
    }
}
