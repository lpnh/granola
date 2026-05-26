use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<p>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/p)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let p: HtmlP = HtmlP::empty().id("paragraph");
///
/// assert_eq!(p.bake(), r#"<p id="paragraph"></p>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let p: HtmlP = HtmlP::new("Lorem ipsum dolor sit amet...🙄");
///
/// assert_eq!(p.bake(), r#"<p>Lorem ipsum dolor sit amet...🙄</p>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <p
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</p>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = PTag, content = Cow<'static, str>)]
pub struct HtmlP<R: PTag = ()> {
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

/// Shorthand for `HtmlP`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let p = p!().id("paragraph");
///
/// assert_eq!(p.bake(), r#"<p id="paragraph"></p>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let p = p!("Lorem ipsum dolor sit amet...🙄");
///
/// assert_eq!(p.bake(), r#"<p>Lorem ipsum dolor sit amet...🙄</p>"#);
/// ```
#[macro_export]
macro_rules! p {
    () => {
        $crate::html::HtmlP::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlP::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlP::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlP::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlP::<()>::new($crate::bake_inline![$($content),+])
    };


    (@recipe $($r:ty),+) => {
        $crate::html::HtmlP::<$crate::cookbook!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlP::<$crate::cookbook!($($r),+)>::new($content)
    };
    (@recipe $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlP::<$crate::cookbook!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@recipe $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlP::<$crate::cookbook!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@recipe $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlP::<$crate::cookbook!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
