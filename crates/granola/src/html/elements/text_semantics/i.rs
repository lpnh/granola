use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait ITag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlI<Self>) -> HtmlI<Self> {
        element
    }
}

impl ITag for () {}

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
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</i>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlI<M: ITag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ITag> HtmlI<M> {
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

/// Shorthand for `HtmlI<()>`.
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
