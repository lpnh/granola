use crate::prelude::*;

/// The recipe for the CSS `border-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-width)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_width = CssDeclaration::from(BorderWidth).content("1.2rem");
///
/// assert_eq!(css_border_width.bake(), "border-width: 1.2rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderWidth;

impl DeclarationRecipe for BorderWidth {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border-width".into()
    }
}
