use crate::prelude::*;

/// The recipe for the CSS `box-shadow` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/box-shadow)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_box_shadow = CssDeclaration::from(BoxShadow).content("none");
///
/// assert_eq!(css_box_shadow.bake(), "box-shadow: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BoxShadow;

impl DeclarationRecipe for BoxShadow {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "box-shadow".into()
    }
}
