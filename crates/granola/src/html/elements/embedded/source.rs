use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait SourceTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
}

impl SourceTag for () {}

/// The HTML `<source />` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let source: HtmlSource = HtmlSource::empty().id("media_or_image_source");
///
/// assert_eq!(source.bake(),
/// r#"<source id="media_or_image_source" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let source: HtmlSource = HtmlSource::new("/videos/flower.mp4").mime_type("video/mp4");
///
/// assert_eq!(source.bake(),
/// r#"<source src="/videos/flower.mp4" type="video/mp4" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <source
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSource<M: SourceTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SourceTag> HtmlSource<M> {
    pub fn new(src: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s.src(src)
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#height)
    pub fn height(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("height", value.to_string());
        self
    }

    /// Applicable media.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#media)
    pub fn media(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("media", value.into());
        self
    }

    /// Image sizes for different page layouts.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#sizes)
    pub fn sizes(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("sizes", value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }

    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#srcset)
    pub fn srcset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("srcset", value);
        self
    }

    /// Type of embedded resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#type)
    pub fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value);
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/source#width)
    pub fn width(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("width", value.to_string());
        self
    }
}
