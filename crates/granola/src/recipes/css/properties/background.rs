mod background_color;
pub use background_color::*;

use crate::prelude::*;

/// The recipe for the CSS `background` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/background)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_background = CssDeclaration::from(Background).content("none");
///
/// assert_eq!(css_background.bake(), "background: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Background;

impl DeclarationRecipe for Background {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "background".into()
    }
}
