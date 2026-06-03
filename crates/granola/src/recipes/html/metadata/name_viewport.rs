use crate::prelude::*;

/// The `name="viewport"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let viewport: HtmlMeta<NameViewport> =
///     HtmlMeta::from_content("width=device-width, initial-scale=1");
///
/// assert_eq!(
///     viewport.bake(),
///     r#"<meta name="viewport" content="width=device-width, initial-scale=1" />"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct NameViewport;

impl MetaRecipe for NameViewport {
    fn specific_attrs_recipe(meta_attrs: &mut MetaAttrs) {
        meta_attrs.name("viewport");
    }
}
