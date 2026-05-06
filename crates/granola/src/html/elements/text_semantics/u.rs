use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<u>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/u)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let u: HtmlU = HtmlU::empty().id("unarticulated_annotation");
///
/// assert_eq!(u.bake(),
/// r#"<u id="unarticulated_annotation"></u>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let wowwd: HtmlU = HtmlU::new("world");
///
/// let hewwo_wowwd = bake_inline!["hewwo, ", wowwd, "!"];
///
/// assert_eq!(hewwo_wowwd,
/// r#"hewwo, <u>world</u>!"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <u
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</u>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = UTag, content = Cow<'static, str>)]
pub struct HtmlU<M: UTag = ()> {
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

/// Shorthand for `HtmlU`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let u = u!().id("unarticulated_annotation");
///
/// assert_eq!(u.bake(),
/// r#"<u id="unarticulated_annotation"></u>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let wowwd = u!("world");
///
/// let hewwo_wowwd = bake_inline!["hewwo, ", wowwd, "!"];
///
/// assert_eq!(hewwo_wowwd,
/// r#"hewwo, <u>world</u>!"#);
/// ```
#[macro_export]
macro_rules! u {
    () => {
        $crate::html::HtmlU::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlU::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlU::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlU::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlU::<()>::new($crate::bake_inline![$($content),+])
    };
}
