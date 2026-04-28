use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait CaptionTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl CaptionTag for () {}

/// The HTML `<caption>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/caption)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let caption: HtmlCaption = HtmlCaption::empty().id("table_caption");
///
/// assert_eq!(caption.bake(),
/// r#"<caption id="table_caption"></caption>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let caption: HtmlCaption = HtmlCaption::new("Our favorites, yours to try.");
///
/// assert_eq!(caption.bake(),
/// r#"<caption>Our favorites, yours to try.</caption>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <caption
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</caption>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlCaption<M: CaptionTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: CaptionTag> HtmlCaption<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }
}

/// Shorthand for `HtmlCaption<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let caption = caption!().id("table_caption");
///
/// assert_eq!(caption.bake(),
/// r#"<caption id="table_caption"></caption>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let caption = caption!("Our favorites, yours to try.");
///
/// assert_eq!(caption.bake(),
/// r#"<caption>Our favorites, yours to try.</caption>"#);
/// ```
#[macro_export]
macro_rules! caption {
    () => {
        $crate::html::HtmlCaption::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlCaption::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlCaption::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlCaption::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlCaption::<()>::new($crate::bake_inline![$($content),+])
    };
}
