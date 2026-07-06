use crate::prelude::*;

/// The recipe for the CSS `grid-template-columns` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/grid-template-columns)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_grid_template_columns = CssDeclaration::from(GridTemplateColumns).content("auto 1fr");
///
/// assert_eq!(
///     css_grid_template_columns.bake(),
///     "grid-template-columns: auto 1fr;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GridTemplateColumns;

impl DeclarationRecipe for GridTemplateColumns {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        "grid-template-columns".into()
    }
}
