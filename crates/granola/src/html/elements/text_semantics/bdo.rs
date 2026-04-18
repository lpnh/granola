use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait BdoTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles:
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl BdoTag for () {}

/// The HTML `<bdo>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdo)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdo: HtmlBdo = HtmlBdo::empty().id("bidirectional_text_override");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo id="bidirectional_text_override"></bdo>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdo: HtmlBdo = HtmlBdo::new("looking-glass").dir("rtl");
///
/// assert_eq!(bdo.bake(),
/// r#"<bdo dir="rtl">looking-glass</bdo>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <bdo
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2, 70) }}</bdo>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlBdo<M: BdoTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BdoTag> HtmlBdo<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }
}
