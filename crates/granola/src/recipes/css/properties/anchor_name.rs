use crate::prelude::*;

/// The recipe for the CSS `anchor-name` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/anchor-name)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_anchor_name = CssDeclaration::from(AnchorName).content("--some-anchor");
///
/// assert_eq!(css_anchor_name.bake(), "anchor-name: --some-anchor;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnchorName;

impl DeclarationRecipe for AnchorName {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "anchor-name".into()
    }
}
