use crate::prelude::*;

/// The `button, input, select, textarea` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selectors_list = CssSelectorsList::from(FormControls);
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
    fn selectors_recipe(selectors: &mut Vec<CssComplexSelector>) {
        selectors.extend([
            "button".into(),
            "input".into(),
            "select".into(),
            "textarea".into(),
        ]);
    }
}
