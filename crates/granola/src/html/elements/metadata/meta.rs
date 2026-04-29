use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait MetaTag: Default + Clone + Debug + 'static {
    fn recipe(element: HtmlMeta<Self>) -> HtmlMeta<Self> {
        element
    }
}

impl MetaTag for () {}

/// The HTML `<meta />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let meta: HtmlMeta = HtmlMeta::empty().id("metadata");
///
/// assert_eq!(meta.bake(),
/// r#"<meta id="metadata" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let meta: HtmlMeta = HtmlMeta::new("noindex, nofollow").name("robots");
///
/// assert_eq!(meta.bake(),
/// r#"<meta content="noindex, nofollow" name="robots" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <meta
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlMeta<M: MetaTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: MetaTag> HtmlMeta<M> {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        let element = Self::default();

        M::recipe(element).content(content)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }

    /// Character encoding declaration.
    ///
    /// Set the `charset` attribute to `utf-8`, the only valid encoding for HTML5 documents.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta#charset)
    pub fn charset(mut self) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("charset", "utf-8");
        self
    }

    /// Value of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/content)
    pub fn content(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("content", value);
        self
    }

    /// Pragma directive.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta/http-equiv)
    pub fn http_equiv(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("http-equiv", value);
        self
    }

    /// Applicable media.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta#media)
    pub fn media(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("media", value);
        self
    }

    /// Metadata name.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta/name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }
}

/// Shorthand for `HtmlMeta<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let meta = meta!().id("metadata");
///
/// assert_eq!(meta.bake(),
/// r#"<meta id="metadata" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let meta = meta!("noindex, nofollow").name("robots");
///
/// assert_eq!(meta.bake(),
/// r#"<meta content="noindex, nofollow" name="robots" />"#);
/// ```
#[macro_export]
macro_rules! meta {
    () => {
        $crate::html::HtmlMeta::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlMeta::<()>::new($content)
    };
}
