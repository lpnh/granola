use crate::prelude::*;

/// The `method="get"` and `formmethod="get"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let form: HtmlForm<Get> = HtmlForm::empty();
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
/// r#"<input formmethod="get" type="submit" value="Search" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Get;

impl FormTag for Get {
    fn recipe(form: HtmlForm<Self>) -> HtmlForm<Self> {
        form.method("get")
    }
}

impl ButtonTag for Get {
    fn recipe<R: ButtonTag>(button: HtmlButton<R>) -> HtmlButton<R> {
        button.formmethod("get")
    }
}

impl InputTag for Get {
    fn recipe<R: InputTag>(input: HtmlInput<R>) -> HtmlInput<R> {
        input.formmethod("get")
    }
}
