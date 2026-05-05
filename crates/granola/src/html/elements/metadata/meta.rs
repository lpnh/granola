use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- attrs -}}
///   {{- specific_attrs }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MetaTag, specific = MetaAttrs)]
pub struct HtmlMeta<M: MetaTag = ()> {
    _marker: PhantomData<M>,
    pub attrs: Attrs,
    pub specific_attrs: MetaAttrs,
}

impl<M: MetaTag> HtmlMeta<M> {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = MetaAttrs::default().content(content);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }
}

/// The HTML `<meta />` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- charset | bake_attr("charset") -}}
/// {{- content | bake_attr("content") -}}
/// {{- http_equiv | bake_attr("http_equiv") -}}
/// {{- media | bake_attr("media") -}}
/// {{- name | bake_attr("name") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct MetaAttrs {
    pub charset: Option<Cow<'static, str>>,
    pub content: Option<Cow<'static, str>>,
    pub http_equiv: Option<Cow<'static, str>>,
    pub media: Option<Cow<'static, str>>,
    pub name: Option<Cow<'static, str>>,
}

pub trait HasMetaAttrs: Sized {
    fn media_attrs_mut(&mut self) -> &mut MetaAttrs;

    /// Character encoding declaration.
    ///
    /// Set the `charset` attribute to `utf-8`, the only valid encoding for HTML5 documents.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta#charset)
    fn charset(mut self) -> Self {
        self.media_attrs_mut().charset = Some("utf-8".into());
        self
    }

    /// Value of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/content)
    fn content(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.media_attrs_mut().content = Some(value.into());
        self
    }

    /// Pragma directive.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta/http-equiv)
    fn http_equiv(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.media_attrs_mut().http_equiv = Some(value.into());
        self
    }

    /// Applicable media.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta#media)
    fn media(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.media_attrs_mut().media = Some(value.into());
        self
    }

    /// Metadata name.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meta/name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.media_attrs_mut().name = Some(value.into());
        self
    }
}

impl HasMetaAttrs for MetaAttrs {
    fn media_attrs_mut(&mut self) -> &mut MetaAttrs {
        self
    }
}

impl HasMetaAttrs for &mut MetaAttrs {
    fn media_attrs_mut(&mut self) -> &mut MetaAttrs {
        self
    }
}

impl<M: MetaTag> HasMetaAttrs for HtmlMeta<M> {
    fn media_attrs_mut(&mut self) -> &mut MetaAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlMeta`.
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
