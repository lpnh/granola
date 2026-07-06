use crate::prelude::*;

/// The recipe for the CSS `tab-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/tab-size)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_tab_size = CssDeclaration::from(TabSize).content("4");
///
/// assert_eq!(css_tab_size.bake(), "tab-size: 4;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TabSize;

impl DeclarationRecipe for TabSize {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "tab-size".into()
    }
}
