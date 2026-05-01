use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait StrongTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlStrong<Self>) -> HtmlStrong<Self> {
        element
    }
}

impl StrongTag for () {}

/// The HTML `<strong>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/strong)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let strong: HtmlStrong = HtmlStrong::empty().id("strong_importance");
///
/// assert_eq!(strong.bake(),
/// r#"<strong id="strong_importance"></strong>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let strong: HtmlStrong = HtmlStrong::new("Do not feed the trolls.");
///
/// assert_eq!(strong.bake(),
/// r#"<strong>Do not feed the trolls.</strong>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <strong
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</strong>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlStrong<M: StrongTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: StrongTag> HtmlStrong<M> {
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

/// Shorthand for `HtmlStrong<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let strong = strong!().id("strong_importance");
///
/// assert_eq!(strong.bake(),
/// r#"<strong id="strong_importance"></strong>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let strong = strong!("Do not feed the trolls.");
///
/// assert_eq!(strong.bake(),
/// r#"<strong>Do not feed the trolls.</strong>"#);
/// ```
#[macro_export]
macro_rules! strong {
    () => {
        $crate::html::HtmlStrong::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlStrong::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlStrong::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlStrong::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlStrong::<()>::new($crate::bake_inline![$($content),+])
    };
}
