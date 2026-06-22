use crate::prelude::*;

/// The `code, kbd, samp, pre` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selectors_list = CssSelectorsList::from(MonospaceSelectors);
///
/// assert_eq!(selectors_list.bake(), "code, kbd, samp, pre");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MonospaceSelectors;

impl SelectorsListRecipe for MonospaceSelectors {
    fn selectors_recipe(selectors: &mut Vec<CssComplexSelector>) {
        selectors.extend(["code".into(), "kbd".into(), "samp".into(), "pre".into()]);
    }
}
