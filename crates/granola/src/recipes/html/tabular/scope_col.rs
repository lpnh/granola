use crate::prelude::*;

/// The `scope="col"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let th = HtmlTh::from(ScopeCol).content("Item");
///
/// assert_eq!(th.bake(), r#"<th scope="col">Item</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScopeCol;

impl ThRecipe for ScopeCol {
    recipe_boilerplate!(ThRecipe);

    fn specific_attrs_recipe(th_attrs: &mut ThAttrs) {
        th_attrs.scope("col");
    }
}

/// The `scope="row"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let th = HtmlTh::from(ScopeRow).content("Hot chocolate");
///
/// assert_eq!(th.bake(), r#"<th scope="row">Hot chocolate</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScopeRow;

impl ThRecipe for ScopeRow {
    recipe_boilerplate!(ThRecipe);

    fn specific_attrs_recipe(th_attrs: &mut ThAttrs) {
        th_attrs.scope("row");
    }
}
