use crate::prelude::*;

/// The `formmethod="post"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let form: HtmlForm<MethodPost> = HtmlForm::from_recipe();
///
/// assert_eq!(form.bake(), r#"<form method="post"></form>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MethodPost;

impl FormRecipe for MethodPost {
    fn specific_attrs_recipe(form_attrs: &mut FormAttrs) {
        form_attrs.method(FormMethod::Post);
    }
}
