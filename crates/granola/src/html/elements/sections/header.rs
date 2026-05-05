use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<header>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/header)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let header: HtmlHeader = HtmlHeader::empty().id("header");
///
/// assert_eq!(header.bake(),
/// r#"<header id="header"></header>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let logo: HtmlA = HtmlA::new("Oats &amp; Ends").href("/");
///
/// let header: HtmlHeader = HtmlHeader::new(bake_newline!(logo));
///
/// assert_eq!(header.bake(),
/// r#"<header>
///   <a href="/">Oats &amp; Ends</a>
/// </header>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <header{{ attrs }}>{{ content | kirei(2) }}</header>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = HeaderTag, content = Cow<'static, str>)]
pub struct HtmlHeader<M: HeaderTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// group, presentation or none
    pub attrs: Attrs,
}

/// Shorthand for `HtmlHeader`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let header = header!().id("header");
///
/// assert_eq!(header.bake(),
/// r#"<header id="header"></header>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let logo = a!("Oats &amp; Ends").href("/");
///
/// let header = header!(@newline logo);
///
/// assert_eq!(header.bake(),
/// r#"<header>
///   <a href="/">Oats &amp; Ends</a>
/// </header>"#);
/// ```
#[macro_export]
macro_rules! header {
    () => {
        $crate::html::HtmlHeader::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlHeader::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlHeader::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlHeader::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlHeader::<()>::new($crate::bake_inline![$($content),+])
    };
}
