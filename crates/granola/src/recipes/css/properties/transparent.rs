use crate::prelude::*;

/// The `transparent` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_background_color: CssBackgroundColor<Transparent> = CssBackgroundColor::from_cookbook();
///
/// assert_eq!(
///     css_background_color.bake(),
///     "background-color: transparent;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transparent;

impl BackgroundColorRecipe for Transparent {
    fn content_recipe(content: &mut Self::Content) {
        *content = "transparent".into();
    }
}
