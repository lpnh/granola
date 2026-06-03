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
/// let body: HtmlBody = HtmlBody::from_cookbook().id("document_body");
///
/// assert_eq!(body.bake(), r#"<body id="document_body"></body>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let body: HtmlBody = HtmlBody::new(bake_newline!("flow content"));
///
/// assert_eq!(
///     body.bake(),
///     r#"<body>
///   flow content
/// </body>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <body
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</body>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BodyRecipe, content = Cow<'static, str>)]
pub struct HtmlBody<R: BodyRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlBody`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!().id("document_body");
///
/// assert_eq!(body.bake(), r#"<body id="document_body"></body>"#);
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

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlBody::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlBody::<$crate::cookbook_type!($($r),+)>::new($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlBody::<$crate::cookbook_type!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlBody::<$crate::cookbook_type!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlBody::<$crate::cookbook_type!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
