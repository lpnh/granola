use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<noscript>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noscript)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let noscript: HtmlNoscript = HtmlNoscript::empty().id("noscript");
///
/// assert_eq!(noscript.bake(),
/// r#"<noscript id="noscript"></noscript>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let noscript: HtmlNoscript = HtmlNoscript::new("It's javascript all the way down");
///
/// assert_eq!(noscript.bake(),
/// r#"<noscript>It's javascript all the way down</noscript>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <noscript{{ attrs }}>{{ content | kirei(2) }}</noscript>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = NoscriptTag, content = Cow<'static, str>)]
pub struct HtmlNoscript<M: NoscriptTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub attrs: Attrs,
}

/// Shorthand for `HtmlNoscript`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let noscript = noscript!().id("noscript");
///
/// assert_eq!(noscript.bake(),
/// r#"<noscript id="noscript"></noscript>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let noscript = noscript!("It's javascript all the way down");
///
/// assert_eq!(noscript.bake(),
/// r#"<noscript>It's javascript all the way down</noscript>"#);
/// ```
#[macro_export]
macro_rules! noscript {
    () => {
        $crate::html::HtmlNoscript::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlNoscript::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlNoscript::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlNoscript::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlNoscript::<()>::new($crate::bake_inline![$($content),+])
    };
}
