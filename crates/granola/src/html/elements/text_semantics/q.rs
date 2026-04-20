use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait QTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl QTag for () {}

/// The HTML `<q>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let q: HtmlQ = HtmlQ::empty().id("inline_quotation");
///
/// assert_eq!(q.bake(),
/// r#"<q id="inline_quotation"></q>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let q: HtmlQ = HtmlQ::new("This element is intended for short quotations");
///
/// assert_eq!(q.bake(),
/// r#"<q>This element is intended for short quotations</q>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <q
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</q>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlQ<M: QTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: QTag> HtmlQ<M> {
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

    /// Link to the source of the quotation or more information about the edit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/q#cite)
    pub fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("cite", value.into());
        self
    }
}
