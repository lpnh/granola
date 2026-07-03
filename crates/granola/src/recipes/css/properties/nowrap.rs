use crate::prelude::*;

/// The `nowrap` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_white_space = CssWhiteSpace::from(Nowrap);
///
/// assert_eq!(css_white_space.bake(), "white-space: nowrap;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Nowrap;

impl WhiteSpaceRecipe for Nowrap {
    recipe_boilerplate!(WhiteSpaceRecipe);

    fn content_recipe() -> Self::Content {
        "nowrap".into()
    }
}
