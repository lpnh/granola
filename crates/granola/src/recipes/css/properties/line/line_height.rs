use crate::prelude::*;

/// The recipe for the CSS `line-height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/line-height)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_line_height = CssDeclaration::from(LineHeight).content("1.25rem");
///
/// assert_eq!(css_line_height.bake(), "line-height: 1.25rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LineHeight;

impl DeclarationRecipe for LineHeight {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "line-height".into()
    }
}
