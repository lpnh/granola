use crate::prelude::*;

/// The recipe for the CSS `overflow-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/overflow-wrap)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_overflow_wrap = CssDeclaration::from(OverflowWrap).content("break-word");
///
/// assert_eq!(css_overflow_wrap.bake(), "overflow-wrap: break-word;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct OverflowWrap;

impl DeclarationRecipe for OverflowWrap {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "overflow-wrap".into()
    }
}
