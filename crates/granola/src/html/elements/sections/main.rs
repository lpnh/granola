use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait MainTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl MainTag for () {}

/// The HTML `<main>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/main)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let main: HtmlMain = HtmlMain::empty().id("main");
///
/// assert_eq!(main.bake(),
/// r#"<main id="main"></main>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let main: HtmlMain = HtmlMain::new("hello, world!");
///
/// assert_eq!(main.bake(),
/// r#"<main>hello, world!</main>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <main
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</main>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlMain<M: MainTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: MainTag> HtmlMain<M> {
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

/// Shorthand for `HtmlMain<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let main = main!().id("main");
///
/// assert_eq!(main.bake(),
/// r#"<main id="main"></main>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let main = main!("hello, world!");
///
/// assert_eq!(main.bake(),
/// r#"<main>hello, world!</main>"#);
/// ```
#[macro_export]
macro_rules! main {
    () => {
        $crate::html::HtmlMain::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlMain::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlMain::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlMain::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlMain::<()>::new($crate::bake_inline![$($content),+])
    };
}
