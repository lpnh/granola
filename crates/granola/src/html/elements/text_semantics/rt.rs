use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait RtTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl RtTag for () {}

/// The HTML `<rt>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rt)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let rt: HtmlRt = HtmlRt::empty().id("ruby_text");
///
/// assert_eq!(rt.bake(),
/// r#"<rt id="ruby_text"></rt>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let tori: HtmlRt = HtmlRt::new("tori");
///
/// assert_eq!(tori.bake(),
/// r#"<rt>tori</rt>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <rt
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</rt>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlRt<M: RtTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: RtTag> HtmlRt<M> {
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

/// Shorthand for `HtmlRt<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let rt = rt!().id("ruby_text");
///
/// assert_eq!(rt.bake(),
/// r#"<rt id="ruby_text"></rt>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tori = rt!("tori");
///
/// assert_eq!(tori.bake(),
/// r#"<rt>tori</rt>"#);
/// ```
#[macro_export]
macro_rules! rt {
    () => {
        $crate::html::HtmlRt::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlRt::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlRt::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlRt::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlRt::<()>::new($crate::bake_inline![$($content),+])
    };
}
