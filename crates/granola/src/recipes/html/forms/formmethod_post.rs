use crate::prelude::*;

/// The `formmethod="post"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let button = HtmlButton::from(FormmethodPost).content("Send");
///
/// let input = HtmlInput::from(FormmethodPost)
///     .input_type(InputType::Submit)
///     .value("Send");
///
/// assert_eq!(button.bake(), r#"<button formmethod="post">Send</button>"#);
/// assert_eq!(
///     input.bake(),
///     r#"<input type="submit" value="Send" formmethod="post" />"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FormmethodPost;

impl ButtonRecipe for FormmethodPost {
    recipe_boilerplate!(ButtonRecipe);

    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.formmethod(FormMethod::Post);
    }
}

impl InputRecipe for FormmethodPost {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.formmethod(FormMethod::Post);
    }
}
