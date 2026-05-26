use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// assert_eq!(
///     summary.bake(),
///     r#"<summary id="disclosure_summary"></summary>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let summary: HtmlSummary = HtmlSummary::new("Don't forget");
///
/// assert_eq!(summary.bake(), r#"<summary>Don't forget</summary>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <summary
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</summary>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SummaryTag, content = Cow<'static, str>)]
pub struct HtmlSummary<R: SummaryTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlSummary`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let summary = summary!().id("disclosure_summary");
///
/// assert_eq!(
///     summary.bake(),
///     r#"<summary id="disclosure_summary"></summary>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let summary = summary!("Don't forget");
///
/// assert_eq!(summary.bake(), r#"<summary>Don't forget</summary>"#);
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
