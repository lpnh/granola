use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait SmallTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlSmall<Self>) -> HtmlSmall<Self> {
        element
    }
}

impl SmallTag for () {}

/// The HTML `<small>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/small)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let small: HtmlSmall = HtmlSmall::empty().id("side_comment");
///
/// assert_eq!(small.bake(),
/// r#"<small id="side_comment"></small>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let unlicense = "This is free and unencumbered software released into the public domain.";
///
/// let small: HtmlSmall = HtmlSmall::new(unlicense);
///
/// assert_eq!(small.bake(),
/// r#"<small>This is free and unencumbered software released into the public domain.</small>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <small
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</small>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSmall<M: SmallTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SmallTag> HtmlSmall<M> {
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

/// Shorthand for `HtmlSmall<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let small = small!().id("side_comment");
///
/// assert_eq!(small.bake(),
/// r#"<small id="side_comment"></small>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let unlicense = "This is free and unencumbered software released into the public domain.";
///
/// let small = small!(unlicense);
///
/// assert_eq!(small.bake(),
/// r#"<small>This is free and unencumbered software released into the public domain.</small>"#);
/// ```
#[macro_export]
macro_rules! small {
    () => {
        $crate::html::HtmlSmall::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSmall::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSmall::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSmall::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSmall::<()>::new($crate::bake_inline![$($content),+])
    };
}
