use crate::prelude::*;

/// The `nowrap` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_white_space: CssWhiteSpace<Nowrap> = CssWhiteSpace::from_cookbook();
///
/// assert_eq!(css_white_space.bake(), "white-space: nowrap;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Nowrap;

impl WhiteSpaceRecipe for Nowrap {
    fn content_recipe(content: &mut Self::Content) {
        *content = "nowrap".into();
    }
}
