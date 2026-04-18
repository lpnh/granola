use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait BdiTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl BdiTag for () {}

/// The HTML `<bdi>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/bdi)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let bdi: HtmlBdi = HtmlBdi::empty().id("bidirectional_isolate");
///
/// assert_eq!(bdi.bake(),
/// r#"<bdi id="bidirectional_isolate"></bdi>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let gal: HtmlBdi = HtmlBdi::new("גל גדות");
///
/// let notification = bake_inline![gal, " liked your post"];
///
/// assert_eq!(notification,
/// r#"<bdi>גל גדות</bdi> liked your post"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <bdi
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2, 70) }}</bdi>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlBdi<M: BdiTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BdiTag> HtmlBdi<M> {
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
