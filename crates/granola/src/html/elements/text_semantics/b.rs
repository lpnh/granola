use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait BTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlB<Self>) -> HtmlB<Self> {
        element
    }
}

impl BTag for () {}

/// The HTML `<b>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/b)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let b: HtmlB = HtmlB::empty().id("bring_attention_to");
///
/// assert_eq!(b.bake(),
/// r#"<b id="bring_attention_to"></b>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let flour: HtmlB = HtmlB::new("flour");
/// let water: HtmlB = HtmlB::new("water");
/// let salt: HtmlB = HtmlB::new("salt");
///
/// let recipe = bake_inline!["Mix ", flour, ", ", water, ", and ", salt, "."];
///
/// assert_eq!(recipe,
/// r#"Mix <b>flour</b>, <b>water</b>, and <b>salt</b>."#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <b
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</b>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlB<M: BTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BTag> HtmlB<M> {
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

/// Shorthand for `HtmlB<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let b = b!().id("bring_attention_to");
///
/// assert_eq!(b.bake(),
/// r#"<b id="bring_attention_to"></b>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let flour = b!("flour");
/// let water = b!("water");
/// let salt = b!("salt");
///
/// let recipe = bake_inline!["Mix ", flour, ", ", water, ", and ", salt, "."];
///
/// assert_eq!(recipe,
/// r#"Mix <b>flour</b>, <b>water</b>, and <b>salt</b>."#);
/// ```
#[macro_export]
macro_rules! b {
    () => {
        $crate::html::HtmlB::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlB::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlB::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlB::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlB::<()>::new($crate::bake_inline![$($content),+])
    };
}
