use crate::prelude::*;

/// The `rel="stylesheet"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let link = HtmlLink::from(RelStylesheet).href("main.css");
///
/// assert_eq!(link.bake(), r#"<link href="main.css" rel="stylesheet" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct RelStylesheet;

impl LinkRecipe for RelStylesheet {
    fn specific_attrs_recipe(link_attrs: &mut LinkAttrs) {
        link_attrs.rel("stylesheet");
    }
}
