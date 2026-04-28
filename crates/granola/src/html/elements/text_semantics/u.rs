use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait UTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl UTag for () {}

/// The HTML `<u>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/u)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let u: HtmlU = HtmlU::empty().id("unarticulated_annotation");
///
/// assert_eq!(u.bake(),
/// r#"<u id="unarticulated_annotation"></u>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let wowwd: HtmlU = HtmlU::new("world");
///
/// let hewwo_wowwd = bake_inline!["hewwo, ", wowwd, "!"];
///
/// assert_eq!(hewwo_wowwd,
/// r#"hewwo, <u>world</u>!"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <u
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</u>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlU<M: UTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: UTag> HtmlU<M> {
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

/// Shorthand for `HtmlU<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let u = u!().id("unarticulated_annotation");
///
/// assert_eq!(u.bake(),
/// r#"<u id="unarticulated_annotation"></u>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let wowwd = u!("world");
///
/// let hewwo_wowwd = bake_inline!["hewwo, ", wowwd, "!"];
///
/// assert_eq!(hewwo_wowwd,
/// r#"hewwo, <u>world</u>!"#);
/// ```
#[macro_export]
macro_rules! u {
    () => {
        $crate::html::HtmlU::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlU::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlU::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlU::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlU::<()>::new($crate::bake_inline![$($content),+])
    };
}
