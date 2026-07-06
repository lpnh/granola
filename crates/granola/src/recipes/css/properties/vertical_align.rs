use crate::prelude::*;

/// The recipe for the CSS `vertical-align` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/vertical-align)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_vertical_align = CssDeclaration::from(VerticalAlign).content("baseline");
///
/// assert_eq!(css_vertical_align.bake(), "vertical-align: baseline;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct VerticalAlign;

impl DeclarationRecipe for VerticalAlign {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "vertical-align".into()
    }
}
