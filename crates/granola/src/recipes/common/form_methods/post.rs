use crate::prelude::*;

/// The `method="post"` and `formmethod="post"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let form: HtmlForm<Post> = HtmlForm::from_recipe();
///
/// assert_eq!(form.bake(),
/// r#"<form method="post"></form>"#);
/// ```
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
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
    fn decoration_recipe<R: FormTag>(form: HtmlForm<R>) -> HtmlForm<R> {
        form.method("post")
    }
}

impl ButtonTag for Post {
    fn decoration_recipe<R: ButtonTag>(button: HtmlButton<R>) -> HtmlButton<R> {
        button.formmethod("post")
    }
}

impl InputTag for Post {
    fn decoration_recipe<R: InputTag>(input: HtmlInput<R>) -> HtmlInput<R> {
        input.formmethod("post")
    }
}
