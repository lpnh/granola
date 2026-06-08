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
/// let embed = HtmlEmbed::new().id("embed_external_content");
///
/// assert_eq!(embed.bake(), r#"<embed id="embed_external_content" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let embed = HtmlEmbed::new()
///     .src("flower.png")
///     .mime_type(MimeType::Png)
///     .width(420)
///     .height(420);
///
/// assert_eq!(
///     embed.bake(),
///     r#"<embed type="image/png" src="flower.png" width="420" height="420" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <embed
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = EmbedRecipe)]
pub struct HtmlEmbed<R: EmbedRecipe = ()> {
    _recipe: PhantomData<R>,
    /// # Permitted ARIA roles
    ///
    /// application, document, img, none, presentation
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: EmbedAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlEmbed {
    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        Self::new().src(src)
    }
}

/// The HTML `<embed />` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/embed#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- mime_type | bake_attr("type") -}}
/// {{- src | bake_attr("src") -}}
/// {{- width | bake_attr("width") -}}
/// {{- height | bake_attr("height") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct EmbedAttrs {
    pub mime_type: Option<Cow<'static, str>>,
    pub src: Option<Cow<'static, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
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
    ///
    /// See [`MimeType`]
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

impl<R: EmbedRecipe> HasEmbedAttrs for HtmlEmbed<R> {
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
/// assert_eq!(embed.bake(), r#"<embed id="embed_external_content" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let embed = embed!(@src "flower.png")
///     .mime_type(MimeType::Png)
///     .width(420)
///     .height(420);
///
/// assert_eq!(
///     embed.bake(),
///     r#"<embed type="image/png" src="flower.png" width="420" height="420" />"#
/// );
/// ```
#[macro_export]
macro_rules! embed {
    () => {
        $crate::html::HtmlEmbed::new()
    };

    (@src $src:expr $(,)?) => {
        $crate::html::HtmlEmbed::from_src($src)
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlEmbed::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
}
