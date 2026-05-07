use crate::prelude::*;

/// The `method="get"` and `formmethod="get"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let form: HtmlForm<Get> = HtmlForm::from_recipe();
///
/// assert_eq!(form.bake(),
/// r#"<form method="get"></form>"#);
/// ```
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let button: HtmlButton<Get> = HtmlButton::new("Search");
///
/// let input: HtmlInput<Get> = HtmlInput::from_type("submit").value("Search");
///
/// assert_eq!(button.bake(),
/// r#"<button formmethod="get">Search</button>"#);
/// assert_eq!(input.bake(),
/// r#"<input type="submit" value="Search" formmethod="get" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Get;

impl FormTag for Get {
    fn specific_attrs_recipe(form_attrs: &mut FormAttrs) {
        form_attrs.method(FormMethod::Get);
    }
}

impl ButtonTag for Get {
    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.formmethod(FormMethod::Get);
    }
}

impl InputTag for Get {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.formmethod(FormMethod::Get);
    }
}
