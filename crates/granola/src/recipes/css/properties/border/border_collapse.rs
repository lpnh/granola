use crate::prelude::*;

/// The recipe for the CSS `border-collapse` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-collapse)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_collapse = CssDeclaration::from(BorderCollapse).content("collapse");
///
/// assert_eq!(css_border_collapse.bake(), "border-collapse: collapse;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderCollapse;

impl DeclarationRecipe for BorderCollapse {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border-collapse".into()
    }
}
