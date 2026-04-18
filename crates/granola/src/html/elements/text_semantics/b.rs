use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait BTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
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
/// >{{ content | kirei(2, 70) }}</b>
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
