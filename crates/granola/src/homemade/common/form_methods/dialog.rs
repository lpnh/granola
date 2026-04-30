use crate::prelude::*;

/// The `method="dialog"` and `formmethod="dialog"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let form: HtmlForm<Dialog> = HtmlForm::empty();
///
/// assert_eq!(form.bake(),
/// r#"<form method="dialog"></form>"#);
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let button: HtmlButton<Dialog> = HtmlButton::new("Ok");
///
/// let input: HtmlInput<Dialog> = HtmlInput::from_type("submit").value("Ok");
///
/// assert_eq!(button.bake(),
/// r#"<button formmethod="dialog">Ok</button>"#);
/// assert_eq!(input.bake(),
/// r#"<input formmethod="dialog" type="submit" value="Ok" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dialog;

impl FormTag for Dialog {
    fn recipe(form: HtmlForm<Self>) -> HtmlForm<Self> {
        form.method("dialog")
    }
}

impl ButtonTag for Dialog {
    fn recipe(form: HtmlButton<Self>) -> HtmlButton<Self> {
        form.formmethod("dialog")
    }
}

impl InputTag for Dialog {
    fn recipe(form: HtmlInput<Self>) -> HtmlInput<Self> {
        form.formmethod("dialog")
    }
}
