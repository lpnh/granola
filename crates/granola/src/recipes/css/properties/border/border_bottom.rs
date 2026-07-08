use crate::prelude::*;

/// The recipe for the CSS `border-bottom` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-bottom)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_bottom = CssDeclaration::from(BorderBottom).content("1px solid black");
///
/// assert_eq!(css_border_bottom.bake(), "border-bottom: 1px solid black;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderBottom;

impl DeclarationRecipe for BorderBottom {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border-bottom".into()
    }
}
