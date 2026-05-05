use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- attrs -}}
///   {{- specific_attrs }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = EmbedTag, specific = EmbedAttrs)]
pub struct HtmlEmbed<M: EmbedTag = ()> {
    _marker: PhantomData<M>,
    /// # Permitted ARIA roles
    ///
    /// application, document, img, none, presentation
    pub attrs: Attrs,
    pub specific_attrs: EmbedAttrs,
}

impl<M: EmbedTag> HtmlEmbed<M> {
    pub fn new(src: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = EmbedAttrs::default().src(src);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }
}

/// The HTML `<embed />` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- height | bake_attr("height") -}}
/// {{- src | bake_attr("src") -}}
/// {{- mime_type | bake_attr("type") -}}
/// {{- width | bake_attr("width") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct EmbedAttrs {
    pub height: Option<u32>,
    pub src: Option<Cow<'static, str>>,
    pub mime_type: Option<Cow<'static, str>>,
    pub width: Option<u32>,
}

pub trait HasEmbedAttrs: Sized {
    fn embed_attrs_mut(&mut self) -> &mut EmbedAttrs;

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#height)
    fn height(mut self, value: u32) -> Self {
        self.embed_attrs_mut().height = Some(value);
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#src)
    fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.embed_attrs_mut().src = Some(value.into());
        self
    }

    /// Type of embedded resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#type)
    fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.embed_attrs_mut().mime_type = Some(value.into());
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#width)
    fn width(mut self, value: u32) -> Self {
        self.embed_attrs_mut().width = Some(value);
        self
    }
}

impl HasEmbedAttrs for EmbedAttrs {
    fn embed_attrs_mut(&mut self) -> &mut EmbedAttrs {
        self
    }
}

impl HasEmbedAttrs for &mut EmbedAttrs {
    fn embed_attrs_mut(&mut self) -> &mut EmbedAttrs {
        self
    }
}

impl<M: EmbedTag> HasEmbedAttrs for HtmlEmbed<M> {
    fn embed_attrs_mut(&mut self) -> &mut EmbedAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlEmbed`.
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

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlDel::<$crate::rec!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $src:expr $(,)?) => {
        $crate::html::HtmlDel::<$crate::rec!($($r),+)>::new($src)
    };
}
