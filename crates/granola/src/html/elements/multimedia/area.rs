use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait AreaTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
}

impl AreaTag for () {}

/// The HTML `<area />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let area: HtmlArea = HtmlArea::empty().id("image_map_area");
///
/// assert_eq!(area.bake(),
/// r#"<area id="image_map_area" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let area: HtmlArea = HtmlArea::new("https://w.wiki/LTnF", "Red triangle")
///     .shape("poly")
///     .coords("300,63,470,357,130,357");
///
/// assert_eq!(area.bake(),
/// r#"<area href="https://w.wiki/LTnF" alt="Red triangle" shape="poly" coords="300,63,470,357,130,357" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <area
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlArea<M: AreaTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: AreaTag> HtmlArea<M> {
    pub fn new(href: impl Into<Cow<'static, str>>, alt: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s.href(href).alt(alt)
    }

    pub fn from_href(href: impl Into<Cow<'static, str>>) -> Self {
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

    /// Replacement text for use when images are not available.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#alt)
    pub fn alt(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("alt", value);
        self
    }

    /// Coordinates for the shape to be created in an image map.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#coords)
    pub fn coords(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("coords", value);
        self
    }

    /// Whether to download the resource instead of navigating to it, and its filename if so.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#download)
    pub fn download(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("download", value);
        self
    }

    /// Address of the hyperlink.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#href)
    pub fn href(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("href", value);
        self
    }

    // NOTE: Include `interestfor` in the future.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#interestfor)

    /// URLs to ping.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#ping)
    pub fn ping(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("ping", value);
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#referrerpolicy)
    pub fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("referrerpolicy", value);
        self
    }

    /// Relationship between the location in the document containing the hyperlink and the destination resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
    pub fn rel(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("rel", value);
        self
    }

    /// The kind of shape to be created in an image map.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#shape)
    pub fn shape(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("shape", value);
        self
    }

    /// Navigable for hyperlink navigation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#target)
    pub fn target(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("target", value);
        self
    }
}

/// ```askama
/// {%- for area in items -%}
/// {{- area -}}
/// {%- if !loop.last %}
/// {% endif -%}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, PartialEq, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct Areas {
    items: Vec<HtmlArea>,
}

impl<I: IntoIterator<Item = HtmlArea>> From<I> for Areas {
    fn from(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl From<HtmlArea> for Areas {
    fn from(area: HtmlArea) -> Self {
        Self { items: vec![area] }
    }
}
