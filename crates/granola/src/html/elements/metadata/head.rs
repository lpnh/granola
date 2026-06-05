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
/// let head = HtmlHead::new().id("document_metadata");
///
/// assert_eq!(head.bake(), r#"<head id="document_metadata"></head>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let charset = HtmlMeta::new().charset();
/// let viewport = HtmlMeta::new()
///     .name("viewport")
///     .content("width=device-width");
/// let title = HtmlTitle::new().content("Document title");
///
/// let head = HtmlHead::new().content(bake_block![charset, viewport, title]);
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
        $crate::html::HtmlHead::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlHead::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlHead::new().content($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content:expr $(,)?) => {
        $crate::html::HtmlHead::new().content($crate::bake_newline!($content))
    };
    (@inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlHead::new().content($crate::bake_inline![$($content),+])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlHead::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlHead::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlHead::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlHead::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlHead::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_inline![$($content),+])
    };
}
