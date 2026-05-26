use crate::prelude::*;

/// The `type="submit"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let button: HtmlButton<TypeSubmit> = HtmlButton::new("Clear");
/// let input: HtmlInput<TypeSubmit> = HtmlInput::from_value("Clear");
///
/// assert_eq!(button.bake(), r#"<button type="submit">Clear</button>"#);
/// assert_eq!(input.bake(), r#"<input type="submit" value="Clear" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TypeSubmit;

impl ButtonRecipe for TypeSubmit {
    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.button_type(ButtonType::Submit);
    }
}

impl InputRecipe for TypeSubmit {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.input_type(InputType::Submit);
    }
}
