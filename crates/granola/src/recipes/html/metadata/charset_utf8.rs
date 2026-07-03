use crate::prelude::*;

/// The `charset="utf-8"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let charset = HtmlMeta::from(CharsetUtf8);
///
/// assert_eq!(charset.bake(), r#"<meta charset="utf-8" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CharsetUtf8;

impl MetaRecipe for CharsetUtf8 {
    fn specific_attrs_recipe() -> MetaAttrs {
        MetaAttrs::default().charset()
    }
}
