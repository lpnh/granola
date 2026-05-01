use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait KbdTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlKbd<Self>) -> HtmlKbd<Self> {
        element
    }
}

impl KbdTag for () {}

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
/// assert_eq!(kbd.bake(),
/// r#"<kbd id="keyboard_input"></kbd>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let kbd: HtmlKbd = HtmlKbd::new("Enter");
///
/// assert_eq!(kbd.bake(),
/// r#"<kbd>Enter</kbd>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <kbd
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</kbd>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlKbd<M: KbdTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: KbdTag> HtmlKbd<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }
}

/// Shorthand for `HtmlKbd<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let kbd = kbd!().id("keyboard_input");
///
/// assert_eq!(kbd.bake(),
/// r#"<kbd id="keyboard_input"></kbd>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let kbd = kbd!("Enter");
///
/// assert_eq!(kbd.bake(),
/// r#"<kbd>Enter</kbd>"#);
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
