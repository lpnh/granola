use crate::prelude::*;

/// The `transparent` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_background_color = CssBackgroundColor::from(Transparent);
///
/// assert_eq!(
///     css_background_color.bake(),
///     "background-color: transparent;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transparent;

impl BackgroundColorRecipe for Transparent {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "transparent".into();
    }
}
