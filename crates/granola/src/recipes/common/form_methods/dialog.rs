use crate::prelude::*;

/// The `method="dialog"` and `formmethod="dialog"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let form: HtmlForm<Dialog> = HtmlForm::from_recipe();
///
/// assert_eq!(form.bake(), r#"<form method="dialog"></form>"#);
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let button: HtmlButton<Dialog> = HtmlButton::new("Ok");
///
/// let input: HtmlInput<Dialog> = HtmlInput::from_type("submit").value("Ok");
///
/// assert_eq!(button.bake(), r#"<button formmethod="dialog">Ok</button>"#);
/// assert_eq!(
///     input.bake(),
///     r#"<input type="submit" value="Ok" formmethod="dialog" />"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dialog;

impl FormRecipe for Dialog {
    fn specific_attrs_recipe(form_attrs: &mut FormAttrs) {
        form_attrs.method(FormMethod::Dialog);
    }
}

impl ButtonRecipe for Dialog {
    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.formmethod(FormMethod::Dialog);
    }
}

impl InputRecipe for Dialog {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.formmethod(FormMethod::Dialog);
    }
}
