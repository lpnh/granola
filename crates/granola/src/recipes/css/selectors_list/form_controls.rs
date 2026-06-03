use crate::prelude::*;

/// The `button, input, select, textarea` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let selectors_list: CssSelectorsList<FormControls> = CssSelectorsList::from_recipe();
///
/// assert_eq!(
///     selectors_list.bake(),
///     "button,
/// input,
/// select,
/// textarea"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FormControls;

impl SelectorsListRecipe for FormControls {
    fn selectors_recipe(selectors: &mut Vec<CssSelector>) {
        selectors.extend([
            "button".into(),
            "input".into(),
            "select".into(),
            "textarea".into(),
        ]);
    }
}
