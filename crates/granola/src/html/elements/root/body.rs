use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait BodyTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlBody<Self>) -> HtmlBody<Self> {
        element
    }
}

impl BodyTag for () {}

/// The HTML `<body>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/body)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let body: HtmlBody = HtmlBody::empty().id("document_body");
///
/// assert_eq!(body.bake(),
/// r#"<body id="document_body"></body>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let body: HtmlBody = HtmlBody::new(bake_newline!("flow content"));
///
/// assert_eq!(body.bake(),
/// r#"<body>
///   flow content
/// </body>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <body
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</body>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlBody<M: BodyTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BodyTag> HtmlBody<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }
}

/// Shorthand for `HtmlBody<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!().id("document_body");
///
/// assert_eq!(body.bake(),
/// r#"<body id="document_body"></body>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!(@newline "flow content");
///
/// assert_eq!(body.bake(),
/// r#"<body>
///   flow content
/// </body>"#);
/// ```
#[macro_export]
macro_rules! body {
    () => {
        $crate::html::HtmlBody::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlBody::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlBody::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlBody::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlBody::<()>::new($crate::bake_inline![$($content),+])
    };
}
