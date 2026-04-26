use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait EmbedTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: application, document, img, none, presentation
    const ROLE: Option<&'static str> = None;
}

impl EmbedTag for () {}

/// The HTML `<embed />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let embed: HtmlEmbed = HtmlEmbed::empty().id("embed_external_content");
///
/// assert_eq!(embed.bake(),
/// r#"<embed id="embed_external_content" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let embed: HtmlEmbed = HtmlEmbed::new("flower.png")
///     .mime_type("image/png")
///     .width(420)
///     .height(420);
///
/// assert_eq!(embed.bake(),
/// r#"<embed src="flower.png" type="image/png" width="420" height="420" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <embed
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlEmbed<M: EmbedTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: EmbedTag> HtmlEmbed<M> {
    pub fn new(src: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s.src(src)
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

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#height)
    pub fn height(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("height", value.to_string());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }

    /// Type of embedded resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#type)
    pub fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value);
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#width)
    pub fn width(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("width", value.to_string());
        self
    }
}

/// Shorthand for `HtmlEmbed<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let embed = embed!().id("embed_external_content");
///
/// assert_eq!(embed.bake(),
/// r#"<embed id="embed_external_content" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let embed = embed!("flower.png")
///     .mime_type("image/png")
///     .width(420)
///     .height(420);
///
/// assert_eq!(embed.bake(),
/// r#"<embed src="flower.png" type="image/png" width="420" height="420" />"#);
/// ```
#[macro_export]
macro_rules! embed {
    () => {
        $crate::html::HtmlEmbed::<()>::empty()
    };
    ($src: expr $(,)?) => {
        $crate::html::HtmlEmbed::<()>::new($src)
    };
}
