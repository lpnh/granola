use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- attrs -}}
///   {{- specific_attrs }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = AreaTag, specific = AreaAttrs)]
pub struct HtmlArea<M: AreaTag = ()> {
    _marker: PhantomData<M>,
    pub attrs: Attrs,
    pub specific_attrs: AreaAttrs,
}

impl<M: AreaTag> HtmlArea<M> {
    pub fn new(href: impl Into<Cow<'static, str>>, alt: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = AreaAttrs::default().href(href).alt(alt);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }

    pub fn from_href(href: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = AreaAttrs::default().href(href);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }
}

/// The HTML `<area>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- alt | bake_attr("alt") -}}
/// {{- coords | bake_attr("coords") -}}
/// {{- download | bake_attr("download") -}}
/// {{- href | bake_attr("href") -}}
/// {{- hreflang | bake_attr("hreflang") -}}
/// {{- lang | bake_attr("lang") -}}
/// {{- ping | bake_attr("ping") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- rel | bake_attr("rel") -}}
/// {{- shape | bake_attr("shape") -}}
/// {{- target | bake_attr("target") -}}
/// {{- mime_type | bake_attr("type") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct AreaAttrs {
    pub alt: Option<Cow<'static, str>>,
    pub coords: Option<Cow<'static, str>>,
    pub download: Option<Cow<'static, str>>,
    pub href: Option<Cow<'static, str>>,
    pub hreflang: Option<Cow<'static, str>>,
    pub lang: Option<Cow<'static, str>>,
    pub ping: Option<Cow<'static, str>>,
    pub referrerpolicy: Option<Cow<'static, str>>,
    pub rel: Option<Cow<'static, str>>,
    pub shape: Option<Cow<'static, str>>,
    pub target: Option<Cow<'static, str>>,
    pub mime_type: Option<Cow<'static, str>>,
}

pub trait HasAreaAttrs: Sized {
    fn area_attrs_mut(&mut self) -> &mut AreaAttrs;

    /// Replacement text for use when images are not available.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#alt)
    fn alt(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().alt = Some(value.into());
        self
    }

    /// Coordinates for the shape to be created in an image map.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#coords)
    fn coords(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().coords = Some(value.into());
        self
    }

    /// Whether to download the resource instead of navigating to it, and its filename if so.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#download)
    fn download(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().download = Some(value.into());
        self
    }

    /// Address of the hyperlink.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#href)
    fn href(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().href = Some(value.into());
        self
    }

    // NOTE: Include `interestfor` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#interestfor)

    /// URLs to ping.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#ping)
    fn ping(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().ping = Some(value.into());
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#referrerpolicy)
    fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().referrerpolicy = Some(value.into());
        self
    }

    /// Relationship between the location in the document containing the hyperlink and the destination resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
    fn rel(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().rel = Some(value.into());
        self
    }

    /// The kind of shape to be created in an image map.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#shape)
    fn shape(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().shape = Some(value.into());
        self
    }

    /// Navigable for hyperlink navigation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#target)
    fn target(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.area_attrs_mut().target = Some(value.into());
        self
    }
}

impl HasAreaAttrs for AreaAttrs {
    fn area_attrs_mut(&mut self) -> &mut AreaAttrs {
        self
    }
}

impl HasAreaAttrs for &mut AreaAttrs {
    fn area_attrs_mut(&mut self) -> &mut AreaAttrs {
        self
    }
}

impl<M: AreaTag> HasAreaAttrs for HtmlArea<M> {
    fn area_attrs_mut(&mut self) -> &mut AreaAttrs {
        &mut self.specific_attrs
    }
}

/// ```askama
/// {%- for area in items %}
/// {{ area -}}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
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

/// Shorthand for `HtmlArea`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let area = area!().id("image_map_area");
///
/// assert_eq!(area.bake(),
/// r#"<area id="image_map_area" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let area = area!("https://w.wiki/LTnF", "Red triangle")
///     .shape("poly")
///     .coords("300,63,470,357,130,357");
///
/// assert_eq!(area.bake(),
/// r#"<area href="https://w.wiki/LTnF" alt="Red triangle" shape="poly" coords="300,63,470,357,130,357" />"#);
/// ```
#[macro_export]
macro_rules! area {
    () => {
        $crate::html::HtmlArea::<()>::empty()
    };
    ($href: expr, $alt: expr $(,)?) => {
        $crate::html::HtmlArea::<()>::new($href, $alt)
    };
    (@from_href $href: expr $(,)?) => {
        $crate::html::HtmlArea::<()>::from_href($href)
    };
}
