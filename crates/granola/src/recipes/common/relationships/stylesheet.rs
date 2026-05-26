use crate::prelude::*;

/// The `rel="stylesheet"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let link: HtmlLink<Stylesheet> = HtmlLink::from_href("main.css");
///
/// assert_eq!(link.bake(), r#"<link href="main.css" rel="stylesheet" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Stylesheet;

impl LinkRecipe for Stylesheet {
    fn specific_attrs_recipe(link_attrs: &mut LinkAttrs) {
        link_attrs.rel("stylesheet");
    }
}
