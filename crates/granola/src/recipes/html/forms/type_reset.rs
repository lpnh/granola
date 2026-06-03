use crate::prelude::*;

/// The `type="reset"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let button: HtmlButton<TypeReset> = HtmlButton::new("Clear");
/// let input: HtmlInput<TypeReset> = HtmlInput::from_value("Clear");
///
/// assert_eq!(button.bake(), r#"<button type="reset">Clear</button>"#);
/// assert_eq!(input.bake(), r#"<input type="reset" value="Clear" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TypeReset;

impl ButtonRecipe for TypeReset {
    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.button_type(ButtonType::Reset);
    }
}

impl InputRecipe for TypeReset {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.input_type(InputType::Reset);
    }
}
