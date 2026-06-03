use crate::prelude::*;

/// The `scope="col"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let th: HtmlTh<ScopeCol> = HtmlTh::new("Item");
///
/// assert_eq!(th.bake(), r#"<th scope="col">Item</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScopeCol;

impl ThRecipe for ScopeCol {
    fn specific_attrs_recipe(th_attrs: &mut ThAttrs) {
        th_attrs.scope("col");
    }
}

/// The `scope="row"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let th: HtmlTh<ScopeRow> = HtmlTh::new("Hot chocolate");
///
/// assert_eq!(th.bake(), r#"<th scope="row">Hot chocolate</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ScopeRow;

impl ThRecipe for ScopeRow {
    fn specific_attrs_recipe(th_attrs: &mut ThAttrs) {
        th_attrs.scope("row");
    }
}
