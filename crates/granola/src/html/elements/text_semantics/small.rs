use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<small>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/small)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let small: HtmlSmall = HtmlSmall::empty().id("side_comment");
///
/// assert_eq!(small.bake(), r#"<small id="side_comment"></small>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let unlicense = "This is free and unencumbered software released into the public domain.";
///
/// let small: HtmlSmall = HtmlSmall::new(unlicense);
///
/// assert_eq!(
///     small.bake(),
///     r#"<small>This is free and unencumbered software released into the public domain.</small>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <small
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</small>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SmallTag, content = Cow<'static, str>)]
pub struct HtmlSmall<R: SmallTag = ()> {
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

/// Shorthand for `HtmlSmall`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let small = small!().id("side_comment");
///
/// assert_eq!(small.bake(), r#"<small id="side_comment"></small>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let unlicense = "This is free and unencumbered software released into the public domain.";
///
/// let small = small!(unlicense);
///
/// assert_eq!(
///     small.bake(),
///     r#"<small>This is free and unencumbered software released into the public domain.</small>"#
/// );
/// ```
#[macro_export]
macro_rules! small {
    () => {
        $crate::html::HtmlSmall::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSmall::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSmall::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSmall::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSmall::<()>::new($crate::bake_inline![$($content),+])
    };
}
