use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<rt>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rt)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let rt: HtmlRt = HtmlRt::empty().id("ruby_text");
///
/// assert_eq!(rt.bake(), r#"<rt id="ruby_text"></rt>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let tori: HtmlRt = HtmlRt::new("tori");
///
/// assert_eq!(tori.bake(), r#"<rt>tori</rt>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <rt
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</rt>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = RtTag, content = Cow<'static, str>)]
pub struct HtmlRt<R: RtTag = ()> {
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

/// Shorthand for `HtmlRt`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let rt = rt!().id("ruby_text");
///
/// assert_eq!(rt.bake(), r#"<rt id="ruby_text"></rt>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tori = rt!("tori");
///
/// assert_eq!(tori.bake(), r#"<rt>tori</rt>"#);
/// ```
#[macro_export]
macro_rules! rt {
    () => {
        $crate::html::HtmlRt::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlRt::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlRt::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlRt::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlRt::<()>::new($crate::bake_inline![$($content),+])
    };
}
