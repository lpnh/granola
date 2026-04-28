use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait HeadTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl HeadTag for () {}

/// The HTML `<head>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/head)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let head: HtmlHead = HtmlHead::empty().id("document_metadata");
///
/// assert_eq!(head.bake(),
/// r#"<head id="document_metadata"></head>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let charset: HtmlMeta = HtmlMeta::empty().charset();
/// let viewport: HtmlMeta = HtmlMeta::empty()
///     .name("viewport")
///     .content("width=device-width");
/// let title: HtmlTitle = HtmlTitle::new("Document title");
///
/// let head: HtmlHead = HtmlHead::new(bake_block![charset, viewport, title]);
///
/// assert_eq!(head.bake(),
/// r#"<head>
///   <meta charset="utf-8" />
///   <meta name="viewport" content="width=device-width" />
///   <title>Document title</title>
/// </head>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <head
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</head>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlHead<M: HeadTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: HeadTag> HtmlHead<M> {
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

/// Shorthand for `HtmlHead<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let head = head!().id("document_metadata");
///
/// assert_eq!(head.bake(),
/// r#"<head id="document_metadata"></head>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let charset = meta!().charset();
/// let viewport = meta!().name("viewport").content("width=device-width");
/// let title = title!("Document title");
///
/// let head = head![charset, viewport, title];
///
/// assert_eq!(head.bake(),
/// r#"<head>
///   <meta charset="utf-8" />
///   <meta name="viewport" content="width=device-width" />
///   <title>Document title</title>
/// </head>"#);
/// ```
#[macro_export]
macro_rules! head {
    () => {
        $crate::html::HtmlHead::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlHead::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlHead::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlHead::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlHead::<()>::new($crate::bake_inline![$($content),+])
    };
}
