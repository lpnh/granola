use crate::prelude::*;

/// The `method="post"` and `formmethod="post"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let form: HtmlForm<Post> = HtmlForm::empty();
///
/// assert_eq!(form.bake(),
/// r#"<form method="post"></form>"#);
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let button: HtmlButton<Post> = HtmlButton::new("Send");
///
/// let input: HtmlInput<Post> = HtmlInput::from_type("submit").value("Send");
///
/// assert_eq!(button.bake(),
/// r#"<button formmethod="post">Send</button>"#);
/// assert_eq!(input.bake(),
/// r#"<input formmethod="post" type="submit" value="Send" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Post;

impl FormTag for Post {
    fn recipe(form: HtmlForm<Self>) -> HtmlForm<Self> {
        form.method("post")
    }
}

impl ButtonTag for Post {
    fn recipe(form: HtmlButton<Self>) -> HtmlButton<Self> {
        form.formmethod("post")
    }
}

impl InputTag for Post {
    fn recipe(form: HtmlInput<Self>) -> HtmlInput<Self> {
        form.formmethod("post")
    }
}
