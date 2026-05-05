use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<bdo>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdo)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdo: HtmlBdo = HtmlBdo::empty().id("bidirectional_text_override");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo id="bidirectional_text_override"></bdo>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdo: HtmlBdo = HtmlBdo::new("looking-glass").dir("rtl");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo dir="rtl">looking-glass</bdo>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <bdo{{ attrs }}>{{ content | kirei(2) }}</bdo>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BdoTag, content = Cow<'static, str>)]
pub struct HtmlBdo<M: BdoTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
}

/// Shorthand for `HtmlBdo`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdo = bdo!().id("bidirectional_text_override");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo id="bidirectional_text_override"></bdo>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdo = bdo!("looking-glass").dir("rtl");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo dir="rtl">looking-glass</bdo>"#);
/// ```
#[macro_export]
macro_rules! bdo {
    () => {
        $crate::html::HtmlBdo::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlBdo::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlBdo::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlBdo::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlBdo::<()>::new($crate::bake_inline![$($content),+])
    };
}
