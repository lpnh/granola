use crate::prelude::*;

/// The `formmethod="dialog"` recipe.
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let button: HtmlButton<FormmethodDialog> = HtmlButton::new("Ok");
///
/// let input: HtmlInput<FormmethodDialog> = HtmlInput::from_type("submit").value("Ok");
///
/// assert_eq!(button.bake(), r#"<button formmethod="dialog">Ok</button>"#);
/// assert_eq!(
///     input.bake(),
///     r#"<input type="submit" value="Ok" formmethod="dialog" />"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FormmethodDialog;

impl ButtonRecipe for FormmethodDialog {
    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.formmethod(FormMethod::Dialog);
    }
}

impl InputRecipe for FormmethodDialog {
    fn specific_attrs_recipe(input_attrs: &mut InputAttrs) {
        input_attrs.formmethod(FormMethod::Dialog);
    }
}
