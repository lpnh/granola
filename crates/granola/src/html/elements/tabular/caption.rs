use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<caption>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/caption)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let caption: HtmlCaption = HtmlCaption::empty().id("table_caption");
///
/// assert_eq!(caption.bake(),
/// r#"<caption id="table_caption"></caption>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let caption: HtmlCaption = HtmlCaption::new("Our favorites, yours to try.");
///
/// assert_eq!(caption.bake(),
/// r#"<caption>Our favorites, yours to try.</caption>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <caption
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</caption>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = CaptionTag, content = Cow<'static, str>)]
pub struct HtmlCaption<M: CaptionTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlCaption`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let caption = caption!().id("table_caption");
///
/// assert_eq!(caption.bake(),
/// r#"<caption id="table_caption"></caption>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let caption = caption!("Our favorites, yours to try.");
///
/// assert_eq!(caption.bake(),
/// r#"<caption>Our favorites, yours to try.</caption>"#);
/// ```
#[macro_export]
macro_rules! caption {
    () => {
        $crate::html::HtmlCaption::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlCaption::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlCaption::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlCaption::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlCaption::<()>::new($crate::bake_inline![$($content),+])
    };
}
