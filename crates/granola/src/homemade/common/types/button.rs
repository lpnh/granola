use crate::prelude::*;

/// The `type="button"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let button: HtmlButton<Button> = HtmlButton::new("Enjoy");
/// let input: HtmlInput<Button> = HtmlInput::from_value("Enjoy");
///
/// assert_eq!(button.bake(),
/// r#"<button type="button">Enjoy</button>"#);
/// assert_eq!(input.bake(),
/// r#"<input type="button" value="Enjoy" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Button;

impl ButtonTag for Button {
    fn recipe(button: HtmlButton<Self>) -> HtmlButton<Self> {
        button.button_type("button")
    }
}

impl InputTag for Button {
    fn recipe(input: HtmlInput<Self>) -> HtmlInput<Self> {
        input.input_type("button")
    }
}
