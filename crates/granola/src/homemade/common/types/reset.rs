use crate::prelude::*;

/// The `type="reset"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
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
    fn recipe(button: HtmlButton<Self>) -> HtmlButton<Self> {
        button.button_type("reset")
    }
}

impl InputTag for Reset {
    fn recipe(input: HtmlInput<Self>) -> HtmlInput<Self> {
        input.input_type("reset")
    }
}
