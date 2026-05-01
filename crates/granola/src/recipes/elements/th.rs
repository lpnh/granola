use crate::prelude::*;

/// The `scope="col"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let th: HtmlTh<Col> = HtmlTh::new("Item");
///
/// assert_eq!(th.bake(),
/// r#"<th scope="col">Item</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Col;

impl ThTag for Col {
    fn recipe(th: HtmlTh<Self>) -> HtmlTh<Self> {
        th.scope("col")
    }
}

/// The `scope="row"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let th: HtmlTh<Row> = HtmlTh::new("Hot chocolate");
///
/// assert_eq!(th.bake(),
/// r#"<th scope="row">Hot chocolate</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Row;

impl ThTag for Row {
    fn recipe(th: HtmlTh<Self>) -> HtmlTh<Self> {
        th.scope("row")
    }
}
