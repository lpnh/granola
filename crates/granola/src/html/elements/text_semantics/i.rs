use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<i>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/i)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let i: HtmlI = HtmlI::empty().id("idiomatic_text");
///
/// assert_eq!(i.bake(),
/// r#"<i id="idiomatic_text"></i>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let voila: HtmlI = HtmlI::new("voilà");
///
/// let quote = bake_inline!["and ", voila, "!"];
///
/// assert_eq!(quote,
/// r#"and <i>voilà</i>!"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <i
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</i>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ITag, content = Cow<'static, str>)]
pub struct HtmlI<R: ITag = ()> {
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

/// Shorthand for `HtmlI`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let i = i!().id("idiomatic_text");
///
/// assert_eq!(i.bake(),
/// r#"<i id="idiomatic_text"></i>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let voila = i!("voilà");
///
/// let quote = bake_inline!["and ", voila, "!"];
///
/// assert_eq!(quote,
/// r#"and <i>voilà</i>!"#);
/// ```
#[macro_export]
macro_rules! i {
    () => {
        $crate::html::HtmlI::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlI::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlI::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlI::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlI::<()>::new($crate::bake_inline![$($content),+])
    };
}
