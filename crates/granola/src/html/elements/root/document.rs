use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML document.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let html_document = HtmlDocument::new();
///
/// assert_eq!(html_document.bake(), r#"<!doctype html><html></html>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let body = HtmlBody::new().content("Hello, world!");
///
/// let html_document = HtmlDocument::new().content(body);
///
/// assert_eq!(
///     html_document.bake(),
///     r#"<!doctype html><html><body>Hello, world!</body></html>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{- HtmlDoctype::new() -}}
/// {{- content | kirei -}}
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = HtmlDocumentRecipe, content = HtmlRoot)]
pub struct HtmlDocument<R: HtmlDocumentRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

/// Shorthand for `HtmlDocument`.
///
/// # Example
///
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let html_document = html_document!();
///
/// assert_eq!(html_document.bake(), r#"<!doctype html><html></html>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!().content("Hello, world!");
///
/// let html_document = html_document!().content(body);
///
/// assert_eq!(
///     html_document.bake(),
///     r#"<!doctype html><html><body>Hello, world!</body></html>"#
/// );
/// ```
#[macro_export]
macro_rules! html_document {
    () => {
        $crate::html::HtmlDocument::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDocument::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDocument::new().content($crate::bake![$first $(, $rest)*])
    };

}
