use crate::prelude::*;

/// The recipe for the CSS `border-top-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-top-width)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_top_width = CssDeclaration::from(BorderTopWidth).content("0.5em");
///
/// assert_eq!(css_border_top_width.bake(), "border-top-width: 0.5em;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderTopWidth;

impl DeclarationRecipe for BorderTopWidth {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border-top-width".into()
    }
}
