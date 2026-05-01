use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait SampTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlSamp<Self>) -> HtmlSamp<Self> {
        element
    }
}

impl SampTag for () {}

/// The HTML `<samp>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/samp)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let samp: HtmlSamp = HtmlSamp::empty().id("sample_output");
///
/// assert_eq!(samp.bake(),
/// r#"<samp id="sample_output"></samp>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let error: HtmlSamp = HtmlSamp::new("No such file or directory");
///
/// assert_eq!(error.bake(),
/// r#"<samp>No such file or directory</samp>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <samp
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</samp>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSamp<M: SampTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SampTag> HtmlSamp<M> {
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

/// Shorthand for `HtmlSamp<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let samp = samp!().id("sample_output");
///
/// assert_eq!(samp.bake(),
/// r#"<samp id="sample_output"></samp>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let error = samp!("No such file or directory");
///
/// assert_eq!(error.bake(),
/// r#"<samp>No such file or directory</samp>"#);
/// ```
#[macro_export]
macro_rules! samp {
    () => {
        $crate::html::HtmlSamp::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSamp::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSamp::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSamp::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSamp::<()>::new($crate::bake_inline![$($content),+])
    };
}
