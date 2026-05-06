use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<rp>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rp)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let rp: HtmlRp = HtmlRp::empty().id("ruby_fallback_parenthesis");
///
/// assert_eq!(rp.bake(),
/// r#"<rp id="ruby_fallback_parenthesis"></rp>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let opening_rp: HtmlRp = HtmlRp::new("(");
/// let rt: HtmlRt = HtmlRt::new("tori");
/// let closing_rp: HtmlRp = HtmlRp::new(")");
///
/// let tori = bake_inline![opening_rp, rt, closing_rp];
///
/// assert_eq!(tori,
/// r#"<rp>(</rp><rt>tori</rt><rp>)</rp>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <rp
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</rp>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = RpTag, content = Cow<'static, str>)]
pub struct HtmlRp<M: RpTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlRp`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let rp = rp!().id("ruby_fallback_parenthesis");
///
/// assert_eq!(rp.bake(),
/// r#"<rp id="ruby_fallback_parenthesis"></rp>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let opening_rp = rp!("(");
/// let rt = rt!("tori");
/// let closing_rp = rp!(")");
///
/// let tori = bake_inline![opening_rp, rt, closing_rp];
///
/// assert_eq!(tori,
/// r#"<rp>(</rp><rt>tori</rt><rp>)</rp>"#);
/// ```
#[macro_export]
macro_rules! rp {
    () => {
        $crate::html::HtmlRp::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlRp::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlRp::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlRp::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlRp::<()>::new($crate::bake_inline![$($content),+])
    };
}
