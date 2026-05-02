use crate::prelude::*;

/// The `type="submit"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let button: HtmlButton<Submit> = HtmlButton::new("Clear");
/// let input: HtmlInput<Submit> = HtmlInput::from_value("Clear");
///
/// assert_eq!(button.bake(),
/// r#"<button type="submit">Clear</button>"#);
/// assert_eq!(input.bake(),
/// r#"<input type="submit" value="Clear" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Submit;

impl ButtonTag for Submit {
    fn decoration_recipe<R: ButtonTag>(button: HtmlButton<R>) -> HtmlButton<R> {
        button.button_type("submit")
    }
}

impl InputTag for Submit {
    fn decoration_recipe<R: InputTag>(input: HtmlInput<R>) -> HtmlInput<R> {
        input.input_type("submit")
    }
}
