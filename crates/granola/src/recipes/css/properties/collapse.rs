use crate::prelude::*;

/// The `collapse` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_collapse = CssBorderCollapse::from(Collapse);
///
/// assert_eq!(css_border_collapse.bake(), "border-collapse: collapse;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Collapse;

impl BorderCollapseRecipe for Collapse {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "collapse".into();
    }
}
