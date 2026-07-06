use crate::prelude::*;

/// The recipe for the CSS `box-sizing` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/box-sizing)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_box_sizing = CssDeclaration::from(BoxSizing).content("border-box");
///
/// assert_eq!(css_box_sizing.bake(), "box-sizing: border-box;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BoxSizing;

impl DeclarationRecipe for BoxSizing {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "box-sizing".into()
    }
}
