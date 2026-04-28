use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait SpanTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl SpanTag for () {}

/// The HTML `<span>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/span)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let span: HtmlSpan = HtmlSpan::empty().id("content_span");
///
/// assert_eq!(span.bake(),
/// r#"<span id="content_span"></span>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let span: HtmlSpan = HtmlSpan::new("aesthetic").class("tracking-widest");
///
/// assert_eq!(span.bake(),
/// r#"<span class="tracking-widest">aesthetic</span>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <span
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</span>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSpan<M: SpanTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SpanTag> HtmlSpan<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }
}

/// Shorthand for `HtmlSpan<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let span = span!().id("content_span");
///
/// assert_eq!(span.bake(),
/// r#"<span id="content_span"></span>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let span = span!("aesthetic").class("tracking-widest");
///
/// assert_eq!(span.bake(),
/// r#"<span class="tracking-widest">aesthetic</span>"#);
/// ```
#[macro_export]
macro_rules! span {
    () => {
        $crate::html::HtmlSpan::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSpan::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSpan::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSpan::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSpan::<()>::new($crate::bake_inline![$($content),+])
    };
}
