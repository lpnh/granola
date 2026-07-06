use crate::prelude::*;

/// The recipe for the CSS `position-anchor` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/position-anchor)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_position_anchor = CssDeclaration::from(PositionAnchor).content("--my-anchor");
///
/// assert_eq!(css_position_anchor.bake(), "position-anchor: --my-anchor;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PositionAnchor;

impl DeclarationRecipe for PositionAnchor {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "position-anchor".into()
    }
}
