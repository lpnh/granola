use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait SubTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlSub<Self>) -> HtmlSub<Self> {
        element
    }
}

impl SubTag for () {}

/// The HTML `<sub>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sub)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let sub: HtmlSub = HtmlSub::empty().id("subscript");
///
/// assert_eq!(sub.bake(),
/// r#"<sub id="subscript"></sub>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let sub: HtmlSub = HtmlSub::new("2");
///
/// let water = bake_inline!["H", sub, "O"];
///
/// assert_eq!(water,
/// r#"H<sub>2</sub>O"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <sub
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</sub>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSub<M: SubTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SubTag> HtmlSub<M> {
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

/// Shorthand for `HtmlSub<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sub = sub!().id("subscript");
///
/// assert_eq!(sub.bake(),
/// r#"<sub id="subscript"></sub>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sub = sub!("2");
///
/// let water = bake_inline!["H", sub, "O"];
///
/// assert_eq!(water,
/// r#"H<sub>2</sub>O"#);
/// ```
#[macro_export]
macro_rules! sub {
    () => {
        $crate::html::HtmlSub::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSub::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSub::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSub::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSub::<()>::new($crate::bake_inline![$($content),+])
    };
}
