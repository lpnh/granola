use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<body>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/body)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let body: HtmlBody = HtmlBody::from_recipe().id("document_body");
///
/// assert_eq!(body.bake(),
/// r#"<body id="document_body"></body>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let body: HtmlBody = HtmlBody::new(bake_newline!("flow content"));
///
/// assert_eq!(body.bake(),
/// r#"<body>
///   flow content
/// </body>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <body
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</body>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BodyTag, content = Cow<'static, str>)]
pub struct HtmlBody<M: BodyTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

/// Shorthand for `HtmlBody<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!().id("document_body");
///
/// assert_eq!(body.bake(),
/// r#"<body id="document_body"></body>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!(@newline "flow content");
///
/// assert_eq!(body.bake(),
/// r#"<body>
///   flow content
/// </body>"#);
/// ```
#[macro_export]
macro_rules! body {
    () => {
        $crate::html::HtmlBody::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlBody::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlBody::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlBody::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlBody::<()>::new($crate::bake_inline![$($content),+])
    };

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlBody::<$crate::rec!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlBody::<$crate::rec!($($r),+)>::new($content)
    };
    (@recipe $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlBody::<$crate::rec!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@recipe $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlBody::<$crate::rec!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@recipe $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlBody::<$crate::rec!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
