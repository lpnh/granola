use crate::prelude::*;

/// The recipe for the CSS `flex-shrink` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/flex-shrink)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_flex_shrink = CssDeclaration::from(FlexShrink).content("0");
///
/// assert_eq!(css_flex_shrink.bake(), "flex-shrink: 0;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FlexShrink;

impl DeclarationRecipe for FlexShrink {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "flex-shrink".into()
    }
}
