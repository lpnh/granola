use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<kbd>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/kbd)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let kbd: HtmlKbd = HtmlKbd::empty().id("keyboard_input");
///
/// assert_eq!(kbd.bake(), r#"<kbd id="keyboard_input"></kbd>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let kbd: HtmlKbd = HtmlKbd::new("Enter");
///
/// assert_eq!(kbd.bake(), r#"<kbd>Enter</kbd>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <kbd
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</kbd>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = KbdTag, content = Cow<'static, str>)]
pub struct HtmlKbd<R: KbdTag = ()> {
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

/// Shorthand for `HtmlKbd`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let kbd = kbd!().id("keyboard_input");
///
/// assert_eq!(kbd.bake(), r#"<kbd id="keyboard_input"></kbd>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let kbd = kbd!("Enter");
///
/// assert_eq!(kbd.bake(), r#"<kbd>Enter</kbd>"#);
/// ```
#[macro_export]
macro_rules! kbd {
    () => {
        $crate::html::HtmlKbd::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlKbd::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlKbd::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlKbd::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlKbd::<()>::new($crate::bake_inline![$($content),+])
    };
}
