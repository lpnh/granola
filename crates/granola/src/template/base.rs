use askama::Template;

use crate::prelude::*;

/// The base template.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, template::*};
///
/// let tmpl = TmplBase::new();
///
/// assert_eq!(
///     tmpl.bake(),
///     r#"<!doctype html>
/// <html></html>"#
/// );
/// ```
///
/// ```rust
/// use granola::{prelude::*, template::*};
///
/// let body = HtmlBody::new().content(bake_newline!("Hello, world!"));
///
/// let tmpl = TmplBase::new().content(body);
///
/// assert_eq!(
///     tmpl.bake(),
///     r#"<!doctype html>
/// <html>
///   <body>
///     Hello, world!
///   </body>
/// </html>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{ HtmlDoctype::new() }}
/// {{ html_root }}
/// ```
#[derive(Debug, Clone, Default, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TmplBase<R: HtmlRecipe = ()> {
    pub html_root: HtmlRoot<R>,
}

impl TmplBase {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<R: HtmlRecipe> TmplBase<R> {
    pub fn content(mut self, content: impl Into<R::Content>) -> Self {
        self.html_root = self.html_root.content(content);
        self
    }

    pub fn from_cookbook() -> Self {
        Self {
            html_root: HtmlRoot::<R>::from_cookbook(),
        }
    }
}
