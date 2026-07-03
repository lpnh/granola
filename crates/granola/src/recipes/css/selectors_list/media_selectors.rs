use crate::prelude::*;

/// The `canvas, img, picture, svg, video` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selectors_list = CssSelectorsList::from(MediaSelectors);
///
/// assert_eq!(selectors_list.bake(), "canvas, img, picture, svg, video");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MediaSelectors;

impl SelectorsListRecipe for MediaSelectors {
    fn selectors_recipe() -> Bake {
        let mut selectors = Bake::default();
        for selector in ["canvas", "img", "picture", "svg", "video"] {
            selectors.fold_in_with(", ", selector);
        }
        selectors
    }
}
