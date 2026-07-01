use crate::prelude::*;

/// The `formmethod="post"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let form = HtmlForm::from(MethodPost);
///
/// assert_eq!(form.bake(), r#"<form method="post"></form>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MethodPost;

impl FormRecipe for MethodPost {
    recipe_boilerplate!(FormRecipe);

    fn specific_attrs_recipe(form_attrs: &mut FormAttrs) {
        form_attrs.method(FormMethod::Post);
    }
}
