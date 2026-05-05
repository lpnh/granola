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
    fn specific_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.button_type(ButtonType::Reset);
    }
}

impl InputTag for Reset {
    fn specific_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.input_type(InputType::Reset);
    }
}
