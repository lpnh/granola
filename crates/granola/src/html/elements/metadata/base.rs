use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait BaseTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
}

impl BaseTag for () {}

/// The HTML `<base />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let base: HtmlBase = HtmlBase::empty().id("document_base_url");
///
/// assert_eq!(base.bake(),
/// r#"<base id="document_base_url" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let base: HtmlBase = HtmlBase::new("https://www.example.com/");
///
/// assert_eq!(base.bake(),
/// r#"<base href="https://www.example.com/" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <base
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlBase<M: BaseTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: BaseTag> HtmlBase<M> {
    pub fn new(href: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s.href(href)
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    /// Document base URL.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base#href)
    pub fn href(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("href", value.into());
        self
    }

    /// Default navigable for hyperlink navigation and form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/base#target)
    pub fn target(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("target", value.into());
        self
    }
}
