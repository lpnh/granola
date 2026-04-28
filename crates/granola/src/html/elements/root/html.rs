use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait HtmlTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = HtmlRootContent;
}

impl HtmlTag for () {}

/// The HTML `<html>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/html)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let html: HtmlRoot = HtmlRoot::empty().id("html_document");
///
/// assert_eq!(html.bake(),
/// r#"<html id="html_document"></html>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let body: HtmlBody = HtmlBody::new(bake_newline!("flow content"));
///
/// let html: HtmlRoot = HtmlRoot::new(body).lang("en");
///
/// assert_eq!(html.bake(),
/// r#"<html lang="en">
///   <body>
///     flow content
///   </body>
/// </html>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let meta: HtmlMeta = HtmlMeta::empty().charset();
/// let head: HtmlHead = HtmlHead::new(bake_newline!(meta));
///
/// let p: HtmlP = HtmlP::new("Hello, world!");
/// let body: HtmlBody = HtmlBody::new(bake_newline!(p));
///
/// let html: HtmlRoot = HtmlRoot::new((head, body));
///
/// assert_eq!(html.bake(),
/// r#"<html>
///   <head>
///     <meta charset="utf-8" />
///   </head>
///   <body>
///     <p>Hello, world!</p>
///   </body>
/// </html>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <html
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</html>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlRoot<M: HtmlTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: HtmlTag> HtmlRoot<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }
}

/// One HTML `<head>` element, followed by one `<body>` element.
///
/// The content of [`HtmlRoot`].
///
/// ```askama
/// {%- if let Some(h) = head -%}
/// {{ h }}
/// {% endif -%}
/// {%- if let Some(b) = body -%}
/// {{ b }}
/// {%- endif -%}
/// ```
#[derive(Default, Debug, Clone, PartialEq, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlRootContent {
    head: Option<HtmlHead>,
    body: Option<HtmlBody>,
}

impl From<(HtmlHead, HtmlBody)> for HtmlRootContent {
    fn from((head, body): (HtmlHead, HtmlBody)) -> Self {
        Self {
            head: Some(head),
            body: Some(body),
        }
    }
}

impl From<HtmlHead> for HtmlRootContent {
    fn from(head: HtmlHead) -> Self {
        Self {
            head: Some(head),
            body: None,
        }
    }
}

impl From<HtmlBody> for HtmlRootContent {
    fn from(body: HtmlBody) -> Self {
        Self {
            head: None,
            body: Some(body),
        }
    }
}

/// Shorthand for `HtmlRoot<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let html = root!().id("html_document");
///
/// assert_eq!(html.bake(),
/// r#"<html id="html_document"></html>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!(@newline "flow content");
///
/// let html = root!(body).lang("en");
///
/// assert_eq!(html.bake(),
/// r#"<html lang="en">
///   <body>
///     flow content
///   </body>
/// </html>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let meta = meta!().charset();
/// let head = head!(@newline meta);
///
/// let p: HtmlP = HtmlP::new("Hello, world!");
/// let body = body!(@newline p);
///
/// let html = root!(head, body);
///
/// assert_eq!(html.bake(),
/// r#"<html>
///   <head>
///     <meta charset="utf-8" />
///   </head>
///   <body>
///     <p>Hello, world!</p>
///   </body>
/// </html>"#);
/// ```
#[macro_export]
macro_rules! root {
    () => {
        $crate::html::HtmlRoot::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlRoot::<()>::new($content)
    };
    ($head: expr, $body: expr $(,)?) => {
        $crate::html::HtmlRoot::<()>::new(($head, $body))
    };
}
