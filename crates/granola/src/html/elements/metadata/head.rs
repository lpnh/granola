use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<head>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/head)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let head: HtmlHead = HtmlHead::from_recipe().id("document_metadata");
///
/// assert_eq!(head.bake(), r#"<head id="document_metadata"></head>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let charset: HtmlMeta = HtmlMeta::empty().charset();
/// let viewport: HtmlMeta = HtmlMeta::empty()
///     .name("viewport")
///     .content("width=device-width");
/// let title: HtmlTitle = HtmlTitle::new("Document title");
///
/// let head: HtmlHead = HtmlHead::new(bake_block![charset, viewport, title]);
///
/// assert_eq!(
///     head.bake(),
///     r#"<head>
///   <meta charset="utf-8" />
///   <meta name="viewport" content="width=device-width" />
///   <title>Document title</title>
/// </head>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <head
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</head>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = HeadRecipe, content = Cow<'static, str>)]
pub struct HtmlHead<R: HeadRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlHead`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let head = head!().id("document_metadata");
///
/// assert_eq!(head.bake(), r#"<head id="document_metadata"></head>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let charset = meta!().charset();
/// let viewport = meta!().name("viewport").content("width=device-width");
/// let title = title!("Document title");
///
/// let head = head![charset, viewport, title];
///
/// assert_eq!(
///     head.bake(),
///     r#"<head>
///   <meta charset="utf-8" />
///   <meta name="viewport" content="width=device-width" />
///   <title>Document title</title>
/// </head>"#
/// );
/// ```
#[macro_export]
macro_rules! head {
    () => {
        $crate::html::HtmlHead::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlHead::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlHead::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlHead::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlHead::<()>::new($crate::bake_inline![$($content),+])
    };

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlHead::<$crate::cookbook!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlHead::<$crate::cookbook!($($r),+)>::new($content)
    };
    (@recipe $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlHead::<$crate::cookbook!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@recipe $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlHead::<$crate::cookbook!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@recipe $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlHead::<$crate::cookbook!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
