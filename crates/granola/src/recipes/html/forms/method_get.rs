use crate::prelude::*;

/// The `formmethod="get"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let form: HtmlForm<MethodGet> = HtmlForm::from_recipe();
///
/// assert_eq!(form.bake(), r#"<form method="get"></form>"#);
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MethodGet;

impl FormRecipe for MethodGet {
    fn specific_attrs_recipe(form_attrs: &mut FormAttrs) {
        form_attrs.method(FormMethod::Get);
    }
}
