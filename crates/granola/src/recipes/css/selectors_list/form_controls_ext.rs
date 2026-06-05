use crate::prelude::*;

/// The `button, input, optgroup, select, textarea` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let selectors_list = CssSelectorsList::from(FormControlsExt);
///
/// assert_eq!(
///     selectors_list.bake(),
///     "button,
/// input,
/// optgroup,
/// select,
/// textarea"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FormControlsExt;

impl SelectorsListRecipe for FormControlsExt {
    fn selectors_recipe(selectors: &mut Vec<CssComplexSelector>) {
        selectors.extend([
            "button".into(),
            "input".into(),
            "optgroup".into(),
            "select".into(),
            "textarea".into(),
        ]);
    }
}
