use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// group, presentation or none
pub trait HeaderTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlHeader<Self>) -> HtmlHeader<Self> {
        element
    }
}

impl HeaderTag for () {}

/// The HTML `<header>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/header)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let header: HtmlHeader = HtmlHeader::empty().id("header");
///
/// assert_eq!(header.bake(),
/// r#"<header id="header"></header>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let logo: HtmlA = HtmlA::new("Oats &amp; Ends").href("/");
///
/// let header: HtmlHeader = HtmlHeader::new(bake_newline!(logo));
///
/// assert_eq!(header.bake(),
/// r#"<header>
///   <a href="/">Oats &amp; Ends</a>
/// </header>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <header
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</header>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlHeader<M: HeaderTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: HeaderTag> HtmlHeader<M> {
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

/// Shorthand for `HtmlHeader<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let header = header!().id("header");
///
/// assert_eq!(header.bake(),
/// r#"<header id="header"></header>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let logo = a!("Oats &amp; Ends").href("/");
///
/// let header = header!(@newline logo);
///
/// assert_eq!(header.bake(),
/// r#"<header>
///   <a href="/">Oats &amp; Ends</a>
/// </header>"#);
/// ```
#[macro_export]
macro_rules! header {
    () => {
        $crate::html::HtmlHeader::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlHeader::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlHeader::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlHeader::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlHeader::<()>::new($crate::bake_inline![$($content),+])
    };
}
