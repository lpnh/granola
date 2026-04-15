use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait TitleTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
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
/// let title: HtmlTitle = HtmlTitle::new("I asked for a title and all I got was this dummy text");
///
/// assert_eq!(title.bake(),
/// r#"<title>I asked for a title and all I got was this dummy text</title>"#);
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
/// >{{ content | kirei(2, 70) }}</title>
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
