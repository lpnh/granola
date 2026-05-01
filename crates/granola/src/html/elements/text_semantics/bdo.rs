use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait BdoTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlBdo<Self>) -> HtmlBdo<Self> {
        element
    }
}

impl BdoTag for () {}

/// The HTML `<bdo>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdo)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdo: HtmlBdo = HtmlBdo::empty().id("bidirectional_text_override");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo id="bidirectional_text_override"></bdo>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdo: HtmlBdo = HtmlBdo::new("looking-glass").dir("rtl");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo dir="rtl">looking-glass</bdo>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <bdo
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</bdo>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlBdo<M: BdoTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BdoTag> HtmlBdo<M> {
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

/// Shorthand for `HtmlBdo<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdo = bdo!().id("bidirectional_text_override");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo id="bidirectional_text_override"></bdo>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdo = bdo!("looking-glass").dir("rtl");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo dir="rtl">looking-glass</bdo>"#);
/// ```
#[macro_export]
macro_rules! bdo {
    () => {
        $crate::html::HtmlBdo::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlBdo::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlBdo::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlBdo::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlBdo::<()>::new($crate::bake_inline![$($content),+])
    };
}
