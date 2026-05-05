use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// <pre{{ attrs }}>{{ content | kirei(0) }}</pre>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = PreTag, content = Cow<'static, str>)]
pub struct HtmlPre<M: PreTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
}

/// Shorthand for `HtmlPre`.
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
