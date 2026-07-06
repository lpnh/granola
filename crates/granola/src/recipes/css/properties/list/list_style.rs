use crate::prelude::*;

/// The recipe for the CSS `list-style` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/list-style)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_list_style = CssDeclaration::from(ListStyle).content("none");
///
/// assert_eq!(css_list_style.bake(), "list-style: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListStyle;

impl DeclarationRecipe for ListStyle {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "list-style".into()
    }
}
