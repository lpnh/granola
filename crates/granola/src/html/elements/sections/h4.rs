use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait H4Tag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: tab, presentation or none
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl H4Tag for () {}

/// The HTML `<h4>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/Heading_Elements)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let h4: HtmlH4 = HtmlH4::empty().id("html_section_heading");
///
/// assert_eq!(h4.bake(),
/// r#"<h4 id="html_section_heading"></h4>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let h4: HtmlH4 = HtmlH4::new("In fable and literature");
///
/// assert_eq!(h4.bake(),
/// r#"<h4>In fable and literature</h4>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <h4
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</h4>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlH4<M: H4Tag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: H4Tag> HtmlH4<M> {
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
