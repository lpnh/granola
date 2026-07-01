use crate::prelude::*;

/// The `type="submit"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let button = HtmlButton::from(TypeSubmit).content("Clear");
/// let input = HtmlInput::from(TypeSubmit).value("Clear");
///
/// assert_eq!(button.bake(), r#"<button type="submit">Clear</button>"#);
/// assert_eq!(input.bake(), r#"<input type="submit" value="Clear" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TypeSubmit;

impl ButtonRecipe for TypeSubmit {
    recipe_boilerplate!(ButtonRecipe);

    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.button_type(ButtonType::Submit);
    }
}

impl InputRecipe for TypeSubmit {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.input_type(InputType::Submit);
    }
}
