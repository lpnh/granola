use crate::prelude::*;

/// The `scope="col"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let th: HtmlTh<Col> = HtmlTh::new("Item");
///
/// assert_eq!(th.bake(), r#"<th scope="col">Item</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Col;

impl ThRecipe for Col {
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
/// let th: HtmlTh<Row> = HtmlTh::new("Hot chocolate");
///
/// assert_eq!(th.bake(), r#"<th scope="row">Hot chocolate</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Row;

impl ThRecipe for Row {
    fn specific_attrs_recipe(th_attrs: &mut ThAttrs) {
        th_attrs.scope("row");
    }
}
