use crate::prelude::*;

/// The `type="reset"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let button: HtmlButton<Reset> = HtmlButton::new("Clear");
/// let input: HtmlInput<Reset> = HtmlInput::from_value("Clear");
///
/// assert_eq!(button.bake(),
/// r#"<button type="reset">Clear</button>"#);
/// assert_eq!(input.bake(),
/// r#"<input type="reset" value="Clear" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Reset;

impl ButtonTag for Reset {
    fn recipe<R: ButtonTag>(button: HtmlButton<R>) -> HtmlButton<R> {
        button.button_type("reset")
    }
}

impl InputTag for Reset {
    fn recipe<R: InputTag>(input: HtmlInput<R>) -> HtmlInput<R> {
        input.input_type("reset")
    }
}
