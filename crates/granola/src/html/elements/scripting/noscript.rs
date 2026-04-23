use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait NoscriptTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl NoscriptTag for () {}

/// The HTML `<noscript>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noscript)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let noscript: HtmlNoscript = HtmlNoscript::empty().id("noscript");
///
/// assert_eq!(noscript.bake(),
/// r#"<noscript id="noscript"></noscript>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let noscript: HtmlNoscript = HtmlNoscript::new("It's javascript all the way down");
///
/// assert_eq!(noscript.bake(),
/// r#"<noscript>It's javascript all the way down</noscript>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <noscript
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</noscript>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlNoscript<M: NoscriptTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: NoscriptTag> HtmlNoscript<M> {
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
