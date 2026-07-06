use crate::prelude::*;

/// The recipe for the CSS `border-inline-start` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-inline-start)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_inline_start =
///     CssDeclaration::from(BorderInlineStart).content("3px solid var(--color-accent)");
///
/// assert_eq!(
///     css_border_inline_start.bake(),
///     "border-inline-start: 3px solid var(--color-accent);"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BorderInlineStart;

impl DeclarationRecipe for BorderInlineStart {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "border-inline-start".into()
    }
}
