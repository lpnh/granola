use crate::prelude::*;

/// The `charset="utf-8"` recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let charset: HtmlMeta<CharsetUtf8> = HtmlMeta::from_recipe();
///
/// assert_eq!(charset.bake(), r#"<meta charset="utf-8" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct CharsetUtf8;

impl MetaRecipe for CharsetUtf8 {
    fn specific_attrs_recipe(meta_attrs: &mut MetaAttrs) {
        meta_attrs.charset();
    }
}
