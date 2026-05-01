use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait PreTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlPre<Self>) -> HtmlPre<Self> {
        element
    }
}

impl PreTag for () {}

/// The HTML `<pre>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/pre)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let pre: HtmlPre = HtmlPre::empty().id("preformatted_text");
///
/// assert_eq!(pre.bake(),
/// r#"<pre id="preformatted_text"></pre>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let ferris_ascii = r#"
///  __________________________
/// &lt; Hello fellow Rustaceans! &gt;
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \"#;
///
/// let pre: HtmlPre = HtmlPre::new(ferris_ascii).role("img").aria_label("ASCII ferris");
///
/// let url: HtmlA = HtmlA::new("ferris-says").href(r#"https://crates.io/crates/ferris-says"#);
/// let cite: HtmlCite = HtmlCite::new(url);
///
/// let ferris_says = bake_block![pre, cite];
///
/// assert_eq!(ferris_says,
/// r#"<pre role="img" aria-label="ASCII ferris">
///  __________________________
/// &lt; Hello fellow Rustaceans! &gt;
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// </pre>
/// <cite><a href="https://crates.io/crates/ferris-says">ferris-says</a></cite>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <pre
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(0) }}</pre>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlPre<M: PreTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: PreTag<Content = Cow<'static, str>>> HtmlPre<M> {
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

/// Shorthand for `HtmlPre<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let pre = pre!().id("preformatted_text");
///
/// assert_eq!(pre.bake(),
/// r#"<pre id="preformatted_text"></pre>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ferris_ascii = r#"
///  __________________________
/// &lt; Hello fellow Rustaceans! &gt;
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \"#;
///
/// let pre = pre!(ferris_ascii).role("img").aria_label("ASCII ferris");
///
/// let url = a!("ferris-says").href(r#"https://crates.io/crates/ferris-says"#);
/// let cite = cite!(url);
///
/// let ferris_says = bake_block![pre, cite];
///
/// assert_eq!(ferris_says,
/// r#"<pre role="img" aria-label="ASCII ferris">
///  __________________________
/// &lt; Hello fellow Rustaceans! &gt;
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// </pre>
/// <cite><a href="https://crates.io/crates/ferris-says">ferris-says</a></cite>"#);
/// ```
#[macro_export]
macro_rules! pre {
    () => {
        $crate::html::HtmlPre::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlPre::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlPre::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlPre::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlPre::<()>::new($crate::bake_inline![$($content),+])
    };
}
