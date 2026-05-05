use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<dd>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dd)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dd: HtmlDd = HtmlDd::empty().id("description_details");
///
/// assert_eq!(dd.bake(),
/// r#"<dd id="description_details"></dd>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt: HtmlDt = HtmlDt::new("Hiraeth");
/// let dd: HtmlDd = HtmlDd::new("A longing for a home that no longer exists, or perhaps never did.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(term,
/// r#"<dt>Hiraeth</dt>
/// <dd>A longing for a home that no longer exists, or perhaps never did.</dd>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <dd{{ attrs }}>{{ content | kirei(2) }}</dd>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DdTag, content = Cow<'static, str>)]
pub struct HtmlDd<M: DdTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub attrs: Attrs,
}

/// Shorthand for `HtmlDd`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dd = dd!().id("description_details");
///
/// assert_eq!(dd.bake(),
/// r#"<dd id="description_details"></dd>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dt = dt!("Hiraeth");
/// let dd = dd!("A longing for a home that no longer exists, or perhaps never did.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(term,
/// r#"<dt>Hiraeth</dt>
/// <dd>A longing for a home that no longer exists, or perhaps never did.</dd>"#);
/// ```
#[macro_export]
macro_rules! dd {
    () => {
        $crate::html::HtmlDd::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDd::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDd::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDd::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDd::<()>::new($crate::bake_inline![$($content),+])
    };
}
