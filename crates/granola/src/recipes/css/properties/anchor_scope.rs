use crate::prelude::*;

/// The recipe for the CSS `anchor-scope` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/anchor-scope)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_anchor_scope = CssDeclaration::from(AnchorScope).content("all");
///
/// assert_eq!(css_anchor_scope.bake(), "anchor-scope: all;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AnchorScope;

impl DeclarationRecipe for AnchorScope {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "anchor-scope".into()
    }
}
