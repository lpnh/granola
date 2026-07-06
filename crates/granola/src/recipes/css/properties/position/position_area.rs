use crate::prelude::*;

/// The recipe for the CSS `position-area` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position-area)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_position_area = CssDeclaration::from(PositionArea).content("top");
///
/// assert_eq!(css_position_area.bake(), "position-area: top;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PositionArea;

impl DeclarationRecipe for PositionArea {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "position-area".into()
    }
}
