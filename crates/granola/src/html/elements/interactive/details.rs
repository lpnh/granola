use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait DetailsTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl DetailsTag for () {}

/// The HTML `<details>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let details: HtmlDetails = HtmlDetails::empty().id("details_disclosure");
///
/// assert_eq!(details.bake(),
/// r#"<details id="details_disclosure"></details>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let summary: HtmlSummary = HtmlSummary::new("Pandora's box");
///
/// let details: HtmlDetails = HtmlDetails::new(bake_block![summary, "Hope"]);
///
/// assert_eq!(details.bake(),
/// r#"<details>
///   <summary>Pandora's box</summary>
///   Hope
/// </details>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <details
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</details>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDetails<M: DetailsTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DetailsTag> HtmlDetails<M> {
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

    /// Name of group of mutually-exclusive details elements.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }

    /// Whether the details are visible.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/details#open)
    pub fn open(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("open");
        }
        self
    }
}

/// Shorthand for `HtmlDetails<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let details = details!().id("details_disclosure");
///
/// assert_eq!(details.bake(),
/// r#"<details id="details_disclosure"></details>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let summary = summary!("Pandora's box");
///
/// let details = details![summary, "Hope"];
///
/// assert_eq!(details.bake(),
/// r#"<details>
///   <summary>Pandora's box</summary>
///   Hope
/// </details>"#);
/// ```
#[macro_export]
macro_rules! details {
    () => {
        $crate::html::HtmlDetails::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDetails::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDetails::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDetails::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDetails::<()>::new($crate::bake_inline![$($content),+])
    };
}
