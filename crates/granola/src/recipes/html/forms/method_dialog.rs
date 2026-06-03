use crate::prelude::*;

/// The `method="dialog"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let form: HtmlForm<MethodDialog> = HtmlForm::from_cookbook();
///
/// assert_eq!(form.bake(), r#"<form method="dialog"></form>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MethodDialog;

impl FormRecipe for MethodDialog {
    fn specific_attrs_recipe(form_attrs: &mut FormAttrs) {
        form_attrs.method(FormMethod::Dialog);
    }
}
