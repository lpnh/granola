use askama::Template;

use crate::prelude::*;

/// The HTML `<!doctype html>`.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Glossary/Doctype)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let doctype: HtmlDoctype = HtmlDoctype::new();
///
/// assert_eq!(doctype.bake(),
/// r#"<!doctype html>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <!doctype html>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDoctype {}

impl HtmlDoctype {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Shorthand for `HtmlDoctype`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let doctype = doctype!();
///
/// assert_eq!(doctype.bake(),
/// r#"<!doctype html>"#);
/// ```
#[macro_export]
macro_rules! doctype {
    () => {
        $crate::html::HtmlDoctype::new()
    };
}
