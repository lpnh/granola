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
/// let pre = HtmlPre::new().id("preformatted_text");
///
/// assert_eq!(pre.bake(), r#"<pre id="preformatted_text"></pre>"#);
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
///           / '-----' \
/// "#;
///
/// let pre = HtmlPre::new()
///     .content(ferris_ascii)
///     .role("img")
///     .aria_label("ASCII ferris");
///
/// let url = HtmlA::new()
///     .content("ferris-says")
///     .href(r#"https://crates.io/crates/ferris-says"#);
/// let cite = HtmlCite::new().content(url);
///
/// let ferris_says = bake![pre, cite];
///
/// assert_eq!(
///     ferris_says,
///     r#"<pre role="img" aria-label="ASCII ferris">
///  __________________________
/// &lt; Hello fellow Rustaceans! &gt;
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// </pre><cite><a href="https://crates.io/crates/ferris-says">ferris-says</a></cite>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <pre
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</pre>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = PreRecipe, content = Cow<'static, str>)]
pub struct HtmlPre<R: PreRecipe = ()> {
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

/// Shorthand for `HtmlPre`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let pre = pre!().id("preformatted_text");
///
/// assert_eq!(pre.bake(), r#"<pre id="preformatted_text"></pre>"#);
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
///           / '-----' \
/// "#;
///
/// let pre = pre!(ferris_ascii).role("img").aria_label("ASCII ferris");
///
/// let url = a!("ferris-says").href(r#"https://crates.io/crates/ferris-says"#);
/// let cite = cite!(url);
///
/// let ferris_says = bake![pre, cite];
///
/// assert_eq!(
///     ferris_says,
///     r#"<pre role="img" aria-label="ASCII ferris">
///  __________________________
/// &lt; Hello fellow Rustaceans! &gt;
///  --------------------------
///         \
///          \
///             _~^~^~_
///         \) /  o o  \ (/
///           '_   -   _'
///           / '-----' \
/// </pre><cite><a href="https://crates.io/crates/ferris-says">ferris-says</a></cite>"#
/// );
/// ```
#[macro_export]
macro_rules! pre {
    () => {
        $crate::html::HtmlPre::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlPre::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlPre::new().content($crate::bake![$first $(, $rest)*])
    };
}
