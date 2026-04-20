use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait SummaryTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
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
