use crate::prelude::*;

/// The `h1, h2, h3, h4` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let selectors_list: CssSelectorsList<Headings> = CssSelectorsList::from_recipe();
///
/// assert_eq!(
///     selectors_list.bake(),
///     "h1,
/// h2,
/// h3,
/// h4"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Headings;

impl SelectorsListRecipe for Headings {
    fn selectors_recipe(selectors: &mut Vec<CssSelector>) {
        selectors.extend(["h1".into(), "h2".into(), "h3".into(), "h4".into()]);
    }
}
