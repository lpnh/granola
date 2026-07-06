use crate::prelude::*;

/// The recipe for the CSS `max-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/max-width)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_max_width = CssDeclaration::from(MaxWidth).content("100%");
///
/// assert_eq!(css_max_width.bake(), "max-width: 100%;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MaxWidth;

impl DeclarationRecipe for MaxWidth {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "max-width".into()
    }
}
