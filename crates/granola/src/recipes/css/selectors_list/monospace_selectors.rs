use crate::prelude::*;

/// The `code, kbd, samp, pre` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let selectors_list: CssSelectorsList<MonospaceSelectors> = CssSelectorsList::from_cookbook();
///
/// assert_eq!(
///     selectors_list.bake(),
///     "code,
/// kbd,
/// samp,
/// pre"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MonospaceSelectors;

impl SelectorsListRecipe for MonospaceSelectors {
    fn selectors_recipe(selectors: &mut Vec<CssSelector>) {
        selectors.extend(["code".into(), "kbd".into(), "samp".into(), "pre".into()]);
    }
}
