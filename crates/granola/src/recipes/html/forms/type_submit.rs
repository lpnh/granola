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

    fn specific_attrs_recipe() -> ButtonAttrs {
        ButtonAttrs::default().button_type(ButtonType::Submit)
    }
}

impl InputRecipe for TypeSubmit {
    fn specific_attrs_recipe() -> InputAttrs {
        InputAttrs::default().input_type(InputType::Submit)
    }
}
