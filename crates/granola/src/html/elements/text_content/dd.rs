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
/// let dd = HtmlDd::new().id("description_details");
///
/// assert_eq!(dd.bake(), r#"<dd id="description_details"></dd>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt = HtmlDt::new().content("Hiraeth");
/// let dd =
///     HtmlDd::new().content("A longing for a home that no longer exists, or perhaps never did.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(
///     term,
///     r#"<dt>Hiraeth</dt>
/// <dd>A longing for a home that no longer exists, or perhaps never did.</dd>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <dd
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</dd>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DdRecipe, content = Cow<'static, str>)]
pub struct HtmlDd<R: DdRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
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
/// assert_eq!(dd.bake(), r#"<dd id="description_details"></dd>"#);
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
/// assert_eq!(
///     term,
///     r#"<dt>Hiraeth</dt>
/// <dd>A longing for a home that no longer exists, or perhaps never did.</dd>"#
/// );
/// ```
#[macro_export]
macro_rules! dd {
    () => {
        $crate::html::HtmlDd::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDd::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDd::new().content($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content:expr $(,)?) => {
        $crate::html::HtmlDd::new().content($crate::bake_newline!($content))
    };
    (@inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlDd::new().content($crate::bake_inline![$($content),+])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlDd::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlDd::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDd::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlDd::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlDd::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_inline![$($content),+])
    };
}
