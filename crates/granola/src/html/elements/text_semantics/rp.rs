use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait RpTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl RpTag for () {}

/// The HTML `<rp>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/rp)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let rp: HtmlRp = HtmlRp::empty().id("ruby_fallback_parenthesis");
///
/// assert_eq!(rp.bake(),
/// r#"<rp id="ruby_fallback_parenthesis"></rp>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let opening_rp: HtmlRp = HtmlRp::new("(");
/// let rt: HtmlRt = HtmlRt::new("tori");
/// let closing_rp: HtmlRp = HtmlRp::new(")");
///
/// let tori = bake_inline![opening_rp, rt, closing_rp];
///
/// assert_eq!(tori,
/// r#"<rp>(</rp><rt>tori</rt><rp>)</rp>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <rp
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2, 70) }}</rp>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlRp<M: RpTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: RpTag> HtmlRp<M> {
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
