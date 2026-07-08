use crate::prelude::*;

/// The `canvas, img, picture, svg, video` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(MediaSelectors);
///
/// assert_eq!(rule.selectors_list, "canvas, img, picture, svg, video");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MediaSelectors;

impl RuleRecipe for MediaSelectors {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma!["canvas", "img", "picture", "svg", "video"]
    }
}
