use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<span>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/span)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let span = HtmlSpan::new().id("content_span");
///
/// assert_eq!(span.bake(), r#"<span id="content_span"></span>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let span = HtmlSpan::new()
///     .content("aesthetic")
///     .class("tracking-widest");
///
/// assert_eq!(
///     span.bake(),
///     r#"<span class="tracking-widest">aesthetic</span>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <span
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</span>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SpanRecipe, content = Bake)]
pub struct HtmlSpan<R: SpanRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlSpan`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let span = span!().id("content_span");
///
/// assert_eq!(span.bake(), r#"<span id="content_span"></span>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let span = span!("aesthetic").class("tracking-widest");
///
/// assert_eq!(
///     span.bake(),
///     r#"<span class="tracking-widest">aesthetic</span>"#
/// );
/// ```
#[macro_export]
macro_rules! span {
    () => {
        $crate::html::HtmlSpan::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlSpan::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSpan::new().content($crate::bake![$first $(, $rest)*])
    };
}
