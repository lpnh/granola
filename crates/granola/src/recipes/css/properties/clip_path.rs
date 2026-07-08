use crate::prelude::*;

/// The recipe for the CSS `clip-path` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/clip-path)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_clip_path = CssDeclaration::from(ClipPath).content("inset(50%)");
///
/// assert_eq!(css_clip_path.bake(), "clip-path: inset(50%);");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ClipPath;

impl DeclarationRecipe for ClipPath {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "clip-path".into()
    }
}
