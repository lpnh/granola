use crate::prelude::*;

/// The `scope="col"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let th = HtmlTh::from(Col).content("Item");
///
/// assert_eq!(th.bake(), r#"<th scope="col">Item</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Col;

impl ThRecipe for Col {
    recipe_boilerplate!(ThRecipe);

    fn specific_attrs_recipe() -> ThAttrs {
        ThAttrs::default().scope("col")
    }
}

/// The `scope="row"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let th = HtmlTh::from(Row).content("Hot chocolate");
///
/// assert_eq!(th.bake(), r#"<th scope="row">Hot chocolate</th>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Row;

impl ThRecipe for Row {
    recipe_boilerplate!(ThRecipe);

    fn specific_attrs_recipe() -> ThAttrs {
        ThAttrs::default().scope("row")
    }
}
