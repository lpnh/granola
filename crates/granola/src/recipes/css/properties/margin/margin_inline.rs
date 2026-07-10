use crate::prelude::*;

/// The recipe for the CSS `margin-inline` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-inline)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_margin_inline = CssDeclaration::from(MarginInline).content("auto");
///
/// assert_eq!(css_margin_inline.bake(), "margin-inline: auto;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MarginInline;

impl DeclarationRecipe for MarginInline {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "margin-inline".into()
    }
}
