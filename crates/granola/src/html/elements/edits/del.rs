use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait DelTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl DelTag for () {}

/// The HTML `<del>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let del: HtmlDel = HtmlDel::empty().id("deleted_text");
///
/// assert_eq!(del.bake(),
/// r#"<del id="deleted_text"></del>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let del: HtmlDel = HtmlDel::new(bake_newline!("try!"))
///     .datetime("2019-11-07")
///     .cite("https://github.com/rust-lang/rust/pull/62672/");
///
/// assert_eq!(del.bake(),
/// r#"<del datetime="2019-11-07" cite="https://github.com/rust-lang/rust/pull/62672/">
///   try!
/// </del>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <del
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</del>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDel<M: DelTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DelTag> HtmlDel<M> {
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
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del#cite)
    pub fn cite(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("cite", value.into());
        self
    }

    /// Date and (optionally) time of the change.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/del#datetime)
    pub fn datetime(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("datetime", value.into());
        self
    }
}
