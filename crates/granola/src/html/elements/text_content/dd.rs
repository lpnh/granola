use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait DdTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl DdTag for () {}

/// The HTML `<dd>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dd)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dd: HtmlDd = HtmlDd::empty().id("description_details");
///
/// assert_eq!(dd.bake(),
/// r#"<dd id="description_details"></dd>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let dt: HtmlDt = HtmlDt::new("Hiraeth");
/// let dd: HtmlDd = HtmlDd::new("A longing for a home that no longer exists, or perhaps never did.");
///
/// let term = bake_block![dt, dd];
///
/// assert_eq!(term,
/// r#"<dt>Hiraeth</dt>
/// <dd>A longing for a home that no longer exists, or perhaps never did.</dd>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <dd
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</dd>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDd<M: DdTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DdTag> HtmlDd<M> {
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
