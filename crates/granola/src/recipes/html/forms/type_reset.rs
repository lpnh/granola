use crate::prelude::*;

/// The `type="reset"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let button = HtmlButton::from(TypeReset).content("Clear");
/// let input = HtmlInput::from(TypeReset).value("Clear");
///
/// assert_eq!(button.bake(), r#"<button type="reset">Clear</button>"#);
/// assert_eq!(input.bake(), r#"<input type="reset" value="Clear" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TypeReset;

impl ButtonRecipe for TypeReset {
    recipe_boilerplate!();

    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.button_type(ButtonType::Reset);
    }
}

impl InputRecipe for TypeReset {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.input_type(InputType::Reset);
    }
}
