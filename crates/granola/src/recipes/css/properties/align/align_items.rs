use crate::prelude::*;

/// The recipe for the CSS `align-items` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/align-items)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_align_items = CssDeclaration::from(AlignItems).value("center");
///
/// assert_eq!(css_align_items.bake(), "align-items: center;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AlignItems;

impl DeclarationRecipe for AlignItems {
    fn property_recipe() -> Bake {
        "align-items".into()
    }
}
