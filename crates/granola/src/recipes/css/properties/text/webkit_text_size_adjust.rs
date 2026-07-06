use crate::prelude::*;

/// The recipe for the CSS `-webkit-text-size-adjust` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-size-adjust)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css = CssDeclaration::from(WebkitTextSizeAdjust).content("100%");
///
/// assert_eq!(css.bake(), "-webkit-text-size-adjust: 100%;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct WebkitTextSizeAdjust;

impl DeclarationRecipe for WebkitTextSizeAdjust {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "-webkit-text-size-adjust".into()
    }
}
