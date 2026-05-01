use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait CiteTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlCite<Self>) -> HtmlCite<Self> {
        element
    }
}

impl CiteTag for () {}

/// The HTML `<cite>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/cite)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let cite: HtmlCite = HtmlCite::empty().id("citation");
///
/// assert_eq!(cite.bake(),
/// r#"<cite id="citation"></cite>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let cite: HtmlCite = HtmlCite::new("Act Without Words I");
///
/// assert_eq!(cite.bake(),
/// r#"<cite>Act Without Words I</cite>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <cite
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</cite>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlCite<M: CiteTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: CiteTag> HtmlCite<M> {
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

/// Shorthand for `HtmlCite<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let cite = cite!().id("citation");
///
/// assert_eq!(cite.bake(),
/// r#"<cite id="citation"></cite>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let cite = cite!("Act Without Words I");
///
/// assert_eq!(cite.bake(),
/// r#"<cite>Act Without Words I</cite>"#);
/// ```
#[macro_export]
macro_rules! cite {
    () => {
        $crate::html::HtmlCite::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlCite::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlCite::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlCite::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlCite::<()>::new($crate::bake_inline![$($content),+])
    };
}
