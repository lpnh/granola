use crate::prelude::*;

/// The recipe for the CSS `margin-inline-end` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-inline-end)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_margin_inline_end = CssDeclaration::from(MarginInlineEnd).content("4px");
///
/// assert_eq!(css_margin_inline_end.bake(), "margin-inline-end: 4px;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarginInlineEnd;

impl DeclarationRecipe for MarginInlineEnd {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "margin-inline-end".into()
    }
}
