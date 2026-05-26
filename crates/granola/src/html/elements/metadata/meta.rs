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
/// assert_eq!(meta.bake(), r#"<meta id="metadata" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let meta: HtmlMeta = HtmlMeta::new("noindex, nofollow").name("robots");
///
/// assert_eq!(
///     meta.bake(),
///     r#"<meta name="robots" content="noindex, nofollow" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <meta
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MetaRecipe)]
pub struct HtmlMeta<R: MetaRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: MetaAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: MetaRecipe> HtmlMeta<R> {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        let mut global_attrs = GlobalAttrs::default();
        R::global_attrs_recipe(&mut global_attrs);

        let mut specific_attrs = MetaAttrs::default().content(content.into());
        R::specific_attrs_recipe(&mut specific_attrs);

        let mut global_aria_attrs = GlobalAriaAttrs::default();
        R::global_aria_attrs_recipe(&mut global_aria_attrs);

        let mut custom_data_attrs = CustomDataAttrs::default();
        R::custom_data_attrs_recipe(&mut custom_data_attrs);

        let mut event_handlers = EventHandlers::default();
        R::event_handlers_recipe(&mut event_handlers);

        Self {
            global_attrs,
            specific_attrs,
            global_aria_attrs,
            custom_data_attrs,
            event_handlers,
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
/// {{- name | bake_attr("name") -}}
/// {{- http_equiv | bake_attr("http_equiv") -}}
/// {{- content | bake_attr("content") -}}
/// {{- media | bake_attr("media") -}}
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
    /// Set the `charset` attribute to `utf-8`, the only valid encoding for
    /// HTML5 documents.
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

impl<R: MetaRecipe> HasMetaAttrs for HtmlMeta<R> {
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
/// assert_eq!(meta.bake(), r#"<meta id="metadata" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let meta = meta!("noindex, nofollow").name("robots");
///
/// assert_eq!(
///     meta.bake(),
///     r#"<meta name="robots" content="noindex, nofollow" />"#
/// );
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
