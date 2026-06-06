use crate::prelude::*;

/// The `formmethod="dialog"` recipe.
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let button = HtmlButton::from(FormmethodDialog).content("Ok");
///
/// let input = HtmlInput::from(FormmethodDialog)
///     .input_type(InputType::Submit)
///     .value("Ok");
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
