use crate::prelude::*;

/// The recipe for the CSS `flex-direction` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex-direction)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_flex_direction = CssDeclaration::from(FlexDirection).content("row");
///
/// assert_eq!(css_flex_direction.bake(), "flex-direction: row;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FlexDirection;

impl DeclarationRecipe for FlexDirection {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "flex-direction".into()
    }
}
