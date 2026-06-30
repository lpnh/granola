use crate::prelude::*;

/// The `solid` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border = CssBorder::from(Solid);
///
/// assert_eq!(css_border.bake(), "border: solid;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Solid;

impl BorderRecipe for Solid {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "solid".into();
    }
}

impl OutlineStyleRecipe for Solid {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "solid".into();
    }
}
