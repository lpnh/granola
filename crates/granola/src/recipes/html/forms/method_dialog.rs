use crate::prelude::*;

/// The `method="dialog"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let form = HtmlForm::from(MethodDialog);
///
/// assert_eq!(form.bake(), r#"<form method="dialog"></form>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MethodDialog;

impl FormRecipe for MethodDialog {
    recipe_boilerplate!(FormRecipe);

    fn specific_attrs_recipe(form_attrs: &mut FormAttrs) {
        form_attrs.method(FormMethod::Dialog);
    }
}
