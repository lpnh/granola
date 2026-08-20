use crate::prelude::*;

/// The recipe for the CSS `flex-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex-wrap)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_flex_wrap = CssDeclaration::from(FlexWrap).value("nowrap");
///
/// assert_eq!(css_flex_wrap.bake(), "flex-wrap: nowrap;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FlexWrap;

impl DeclarationRecipe for FlexWrap {
    fn property_recipe() -> Bake {
        "flex-wrap".into()
    }
}
