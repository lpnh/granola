use askama::Template;
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait HtmlTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = HtmlRootContent;
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
/// let body: HtmlBody = HtmlBody::new("content");
///
/// let html: HtmlRoot = HtmlRoot::new(body).lang("en");
///
/// assert_eq!(html.bake(),
/// r#"<html lang="en"><body>content</body></html>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <html
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
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
    pub specific_attrs: SpecificAttrs,
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

impl From<HtmlBody> for HtmlRootContent {
    fn from(body: HtmlBody) -> Self {
        Self {
            head: None,
            body: Some(body),
        }
    }
}
