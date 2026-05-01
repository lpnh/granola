use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait SupTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlSup<Self>) -> HtmlSup<Self> {
        element
    }
}

impl SupTag for () {}

/// The HTML `<sup>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sup)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let sup: HtmlSup = HtmlSup::empty().id("superscript");
///
/// assert_eq!(sup.bake(),
/// r#"<sup id="superscript"></sup>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let sup: HtmlSup = HtmlSup::new("e");
///
/// let anniv = bake_inline!["100", sup, " anniversaire"];
///
/// assert_eq!(anniv,
/// r#"100<sup>e</sup> anniversaire"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <sup
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</sup>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSup<M: SupTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SupTag> HtmlSup<M> {
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

/// Shorthand for `HtmlSup<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sup = sup!().id("superscript");
///
/// assert_eq!(sup.bake(),
/// r#"<sup id="superscript"></sup>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sup = sup!("e");
///
/// let anniv = bake_inline!["100", sup, " anniversaire"];
///
/// assert_eq!(anniv,
/// r#"100<sup>e</sup> anniversaire"#);
/// ```
#[macro_export]
macro_rules! sup {
    () => {
        $crate::html::HtmlSup::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSup::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSup::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSup::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSup::<()>::new($crate::bake_inline![$($content),+])
    };
}
