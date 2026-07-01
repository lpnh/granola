use crate::prelude::*;

/// The `formmethod="get"` recipe.
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let button = HtmlButton::from(FormmethodGet).content("Search");
///
/// let input = HtmlInput::from(FormmethodGet)
///     .input_type(InputType::Submit)
///     .value("Search");
///
/// assert_eq!(button.bake(), r#"<button formmethod="get">Search</button>"#);
/// assert_eq!(
///     input.bake(),
///     r#"<input type="submit" value="Search" formmethod="get" />"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FormmethodGet;

impl ButtonRecipe for FormmethodGet {
    recipe_boilerplate!(ButtonRecipe);

    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.formmethod(FormMethod::Get);
    }
}

impl InputRecipe for FormmethodGet {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.formmethod(FormMethod::Get);
    }
}
