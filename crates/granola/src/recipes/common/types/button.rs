use crate::prelude::*;

/// The `type="button"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let button: HtmlButton<Button> = HtmlButton::new("Enjoy");
/// let input: HtmlInput<Button> = HtmlInput::from_value("Enjoy");
///
/// assert_eq!(button.bake(), r#"<button type="button">Enjoy</button>"#);
/// assert_eq!(input.bake(), r#"<input type="button" value="Enjoy" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Button;

impl ButtonRecipe for Button {
    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.button_type(ButtonType::Button);
    }
}

impl InputRecipe for Button {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.input_type(InputType::Button);
    }
}
