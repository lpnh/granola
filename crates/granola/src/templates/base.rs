use askama::Template;

use crate::prelude::*;

/// The base template.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, templates::*};
///
/// let tmpl: TmplBase = TmplBase::empty();
///
/// assert_eq!(
///     tmpl.bake(),
///     r#"<!doctype html>
/// <html></html>"#
/// );
/// ```
///
/// ```rust
/// use granola::{prelude::*, templates::*};
///
/// let body: HtmlBody = HtmlBody::new(bake_newline!("Hello, world!"));
///
/// let tmpl: TmplBase = TmplBase::new(body);
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

impl<R: HtmlRecipe> TmplBase<R> {
    pub fn new(content: impl Into<R::Content>) -> Self {
        Self {
            html_root: HtmlRoot::<R>::new(content),
        }
    }

    pub fn empty() -> Self {
        Self {
            html_root: HtmlRoot::<R>::empty(),
        }
    }

    pub fn from_recipe() -> Self {
        Self {
            html_root: HtmlRoot::<R>::from_recipe(),
        }
    }
}
