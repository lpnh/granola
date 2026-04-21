use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait DivTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl DivTag for () {}

/// The HTML `<div>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/div)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let div: HtmlDiv = HtmlDiv::empty().id("content_division");
///
/// assert_eq!(div.bake(),
/// r#"<div id="content_division"></div>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let save: HtmlButton = HtmlButton::new("Save");
/// let cancel: HtmlButton = HtmlButton::new("Cancel").button_type("button");
///
/// let content = bake_block![save, cancel];
///
/// let div: HtmlDiv = HtmlDiv::new(content).class("flex justify-end gap-2");
///
/// assert_eq!(div.bake(),
/// r#"<div class="flex justify-end gap-2">
///   <button>Save</button>
///   <button type="button">Cancel</button>
/// </div>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <div
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</div>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDiv<M: DivTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DivTag> HtmlDiv<M> {
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
