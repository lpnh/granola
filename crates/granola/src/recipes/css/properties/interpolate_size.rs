use crate::prelude::*;

/// The recipe for the CSS `interpolate-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/interpolate-size)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_interpolate_size = CssDeclaration::from(InterpolateSize).content("allow-keywords");
///
/// assert_eq!(
///     css_interpolate_size.bake(),
///     "interpolate-size: allow-keywords;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InterpolateSize;

impl DeclarationRecipe for InterpolateSize {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "interpolate-size".into()
    }
}
