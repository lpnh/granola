use crate::prelude::*;

/// The recipe for the CSS `cursor` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/cursor)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_cursor = CssDeclaration::from(Cursor).content("pointer");
///
/// assert_eq!(css_cursor.bake(), "cursor: pointer;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Cursor;

impl DeclarationRecipe for Cursor {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "cursor".into()
    }
}
