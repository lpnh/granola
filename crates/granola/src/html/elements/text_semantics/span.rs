use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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
/// let span: HtmlSpan = HtmlSpan::empty().id("content_span");
///
/// assert_eq!(span.bake(),
/// r#"<span id="content_span"></span>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let span: HtmlSpan = HtmlSpan::new("aesthetic").class("tracking-widest");
///
/// assert_eq!(span.bake(),
/// r#"<span class="tracking-widest">aesthetic</span>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <span{{ attrs }}>{{ content | kirei(2) }}</span>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SpanTag, content = Cow<'static, str>)]
pub struct HtmlSpan<M: SpanTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
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
/// assert_eq!(span.bake(),
/// r#"<span id="content_span"></span>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let span = span!("aesthetic").class("tracking-widest");
///
/// assert_eq!(span.bake(),
/// r#"<span class="tracking-widest">aesthetic</span>"#);
/// ```
#[macro_export]
macro_rules! span {
    () => {
        $crate::html::HtmlSpan::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSpan::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSpan::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSpan::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSpan::<()>::new($crate::bake_inline![$($content),+])
    };
}
