use crate::prelude::*;

/// The `type="button"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
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
    fn specific_recipe(specific: &mut SpecificAttrs) {
        specific.set_attr("type", ButtonType::Button);
    }
}

impl InputTag for Button {
    fn specific_recipe(specific: &mut SpecificAttrs) {
        specific.set_attr("type", InputType::Button);
    }
}
