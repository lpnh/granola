use crate::prelude::*;

/// The `formmethod="get"` recipe.
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let button: HtmlButton<FormmethodGet> = HtmlButton::new("Search");
///
/// let input: HtmlInput<FormmethodGet> = HtmlInput::from_type("submit").value("Search");
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
    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.formmethod(FormMethod::Get);
    }
}

impl InputRecipe for FormmethodGet {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.formmethod(FormMethod::Get);
    }
}
