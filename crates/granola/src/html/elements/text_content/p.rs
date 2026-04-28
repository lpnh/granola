use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait PTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl PTag for () {}

/// The HTML `<p>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/p)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let p: HtmlP = HtmlP::empty().id("paragraph");
///
/// assert_eq!(p.bake(),
/// r#"<p id="paragraph"></p>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let p: HtmlP = HtmlP::new("Lorem ipsum dolor sit amet...🙄");
///
/// assert_eq!(p.bake(),
/// r#"<p>Lorem ipsum dolor sit amet...🙄</p>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <p
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</p>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlP<M: PTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: PTag> HtmlP<M> {
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

/// Shorthand for `HtmlP<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let p = p!().id("paragraph");
///
/// assert_eq!(p.bake(),
/// r#"<p id="paragraph"></p>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let p = p!("Lorem ipsum dolor sit amet...🙄");
///
/// assert_eq!(p.bake(),
/// r#"<p>Lorem ipsum dolor sit amet...🙄</p>"#);
/// ```
#[macro_export]
macro_rules! p {
    () => {
        $crate::html::HtmlP::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlP::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlP::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlP::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlP::<()>::new($crate::bake_inline![$($content),+])
    };
}
