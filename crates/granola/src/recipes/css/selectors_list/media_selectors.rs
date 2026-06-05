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
/// assert_eq!(
///     selectors_list.bake(),
///     "canvas,
/// img,
/// picture,
/// svg,
/// video"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MediaSelectors;

impl SelectorsListRecipe for MediaSelectors {
    fn selectors_recipe(selectors: &mut Vec<CssComplexSelector>) {
        selectors.extend([
            "canvas".into(),
            "img".into(),
            "picture".into(),
            "svg".into(),
            "video".into(),
        ]);
    }
}
