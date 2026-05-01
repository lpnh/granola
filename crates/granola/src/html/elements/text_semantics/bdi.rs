use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait BdiTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlBdi<Self>) -> HtmlBdi<Self> {
        element
    }
}

impl BdiTag for () {}

/// The HTML `<bdi>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdi)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdi: HtmlBdi = HtmlBdi::empty().id("bidirectional_isolate");
///
/// assert_eq!(bdi.bake(),
/// r#"<bdi id="bidirectional_isolate"></bdi>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let gal: HtmlBdi = HtmlBdi::new("גל גדות");
///
/// let notification = bake_inline![gal, " liked your post"];
///
/// assert_eq!(notification,
/// r#"<bdi>גל גדות</bdi> liked your post"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <bdi
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</bdi>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlBdi<M: BdiTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BdiTag> HtmlBdi<M> {
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

/// Shorthand for `HtmlBdi<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let bdi = bdi!().id("bidirectional_isolate");
///
/// assert_eq!(bdi.bake(),
/// r#"<bdi id="bidirectional_isolate"></bdi>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let gal = bdi!("גל גדות");
///
/// let notification = bake_inline![gal, " liked your post"];
///
/// assert_eq!(notification,
/// r#"<bdi>גל גדות</bdi> liked your post"#);
/// ```
#[macro_export]
macro_rules! bdi {
    () => {
        $crate::html::HtmlBdi::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlBdi::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlBdi::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlBdi::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlBdi::<()>::new($crate::bake_inline![$($content),+])
    };
}
