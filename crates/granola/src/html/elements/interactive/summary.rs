use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let summary = HtmlSummary::new().id("disclosure_summary");
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
/// let summary = HtmlSummary::new().content("Don't forget");
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
/// >{{ content | kirei }}</summary>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SummaryRecipe, content = Bake)]
pub struct HtmlSummary<R: SummaryRecipe = ()> {
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
        $crate::html::HtmlSummary::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlSummary::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSummary::new().content($crate::bake![$first $(, $rest)*])
    };

}
