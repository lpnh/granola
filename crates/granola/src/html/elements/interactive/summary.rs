use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait SummaryTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlSummary<Self>) -> HtmlSummary<Self> {
        element
    }
}

impl SummaryTag for () {}

/// The HTML `<summary>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/summary)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let summary: HtmlSummary = HtmlSummary::empty().id("disclosure_summary");
///
/// assert_eq!(summary.bake(),
/// r#"<summary id="disclosure_summary"></summary>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let summary: HtmlSummary = HtmlSummary::new("Don't forget");
///
/// assert_eq!(summary.bake(),
/// r#"<summary>Don't forget</summary>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <summary
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</summary>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSummary<M: SummaryTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SummaryTag> HtmlSummary<M> {
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

/// Shorthand for `HtmlSummary<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let summary = summary!().id("disclosure_summary");
///
/// assert_eq!(summary.bake(),
/// r#"<summary id="disclosure_summary"></summary>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let summary = summary!("Don't forget");
///
/// assert_eq!(summary.bake(),
/// r#"<summary>Don't forget</summary>"#);
/// ```
#[macro_export]
macro_rules! summary {
    () => {
        $crate::html::HtmlSummary::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSummary::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSummary::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSummary::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSummary::<()>::new($crate::bake_inline![$($content),+])
    };
}
