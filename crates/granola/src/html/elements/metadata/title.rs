use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait TitleTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl TitleTag for () {}

/// The HTML `<title>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/title)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let title: HtmlTitle = HtmlTitle::empty().id("document_title");
///
/// assert_eq!(title.bake(),
/// r#"<title id="document_title"></title>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let title: HtmlTitle = HtmlTitle::new("On the unabashed art of self-referential examples");
///
/// assert_eq!(title.bake(),
/// r#"<title>On the unabashed art of self-referential examples</title>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <title
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</title>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTitle<M: TitleTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TitleTag> HtmlTitle<M> {
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

/// Shorthand for `HtmlTitle<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let title = title!().id("document_title");
///
/// assert_eq!(title.bake(),
/// r#"<title id="document_title"></title>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let title = title!("On the unabashed art of self-referential examples");
///
/// assert_eq!(title.bake(),
/// r#"<title>On the unabashed art of self-referential examples</title>"#);
/// ```
#[macro_export]
macro_rules! title {
    () => {
        $crate::html::HtmlTitle::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTitle::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTitle::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTitle::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTitle::<()>::new($crate::bake_inline![$($content),+])
    };
}
