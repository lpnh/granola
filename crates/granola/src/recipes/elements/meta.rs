use crate::prelude::*;

/// The `charset="utf-8"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let charset: HtmlMeta<Charset> = HtmlMeta::from_recipe();
///
/// assert_eq!(charset.bake(),
/// r#"<meta charset="utf-8" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Charset;

impl MetaTag for Charset {
    fn specific_attrs_recipe(meta_attrs: &mut MetaAttrs) {
        meta_attrs.charset();
    }
}

/// The `name="viewport"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let viewport: HtmlMeta<Viewport> = HtmlMeta::new("width=device-width, initial-scale=1");
///
/// assert_eq!(viewport.bake(),
/// r#"<meta name="viewport" content="width=device-width, initial-scale=1" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Viewport;

impl MetaTag for Viewport {
    fn specific_attrs_recipe(meta_attrs: &mut MetaAttrs) {
        meta_attrs.name("viewport");
    }
}

/// The `name="robots"` recipe
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let robots: HtmlMeta<Robots> = HtmlMeta::new("noindex, nofollow");
///
/// assert_eq!(robots.bake(),
/// r#"<meta name="robots" content="noindex, nofollow" />"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Robots;

impl MetaTag for Robots {
    fn specific_attrs_recipe(meta_attrs: &mut MetaAttrs) {
        meta_attrs.name("robots");
    }
}
