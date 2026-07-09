use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let area = HtmlArea::new().id("image_map_area");
///
/// assert_eq!(area.bake(), r#"<area id="image_map_area" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let area = HtmlArea::new()
///     .href("https://w.wiki/LTnF")
///     .alt("Red triangle")
///     .shape("poly")
///     .coords("300,63,470,357,130,357");
///
/// assert_eq!(
///     area.bake_pretty(),
///     r#"<area
///   shape="poly"
///   coords="300,63,470,357,130,357"
///   href="https://w.wiki/LTnF"
///   alt="Red triangle"
/// />
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <area
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = AreaRecipe)]
pub struct HtmlArea<R: AreaRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: AreaAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlArea {
    pub fn from_href_alt(href: impl Into<Bake>, alt: impl Into<Bake>) -> Self {
        Self::new().href(href).alt(alt)
    }

    pub fn from_href(href: impl Into<Bake>) -> Self {
        Self::new().href(href)
    }
}

/// The HTML `<area>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- shape | bake_attr("shape") -}}
/// {{- coords | bake_attr("coords") -}}
/// {{- href | bake_attr("href") -}}
/// {{- alt | bake_attr("alt") -}}
/// {{- download | bake_attr("download") -}}
/// {{- hreflang | bake_attr("hreflang") -}}
/// {{- lang | bake_attr("lang") -}}
/// {{- ping | bake_attr("ping") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- rel | bake_attr("rel") -}}
/// {{- target | bake_attr("target") -}}
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct AreaAttrs {
    pub shape: Option<Bake>,
    pub coords: Option<Bake>,
    pub href: Option<Bake>,
    pub alt: Option<Bake>,
    pub download: Option<Bake>,
    pub hreflang: Option<Bake>,
    pub lang: Option<Bake>,
    pub ping: Option<Bake>,
    pub referrerpolicy: Option<Bake>,
    pub rel: Option<Bake>,
    pub target: Option<Bake>,
}

pub trait HasAreaAttrs: Sized {
    fn area_attrs_mut(&mut self) -> &mut AreaAttrs;

    /// Replacement text for use when images are not available.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#alt)
    fn alt(mut self, value: impl Into<Bake>) -> Self {
        self.area_attrs_mut().alt = Some(value.into());
        self
    }

    /// Coordinates for the shape to be created in an image map.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#coords)
    fn coords(mut self, value: impl Into<Bake>) -> Self {
        self.area_attrs_mut().coords = Some(value.into());
        self
    }

    /// Whether to download the resource instead of navigating to it, and its
    /// filename if so.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#download)
    fn download(mut self, value: impl Into<Bake>) -> Self {
        self.area_attrs_mut().download = Some(value.into());
        self
    }

    /// Address of the hyperlink.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#href)
    fn href(mut self, value: impl Into<Bake>) -> Self {
        self.area_attrs_mut().href = Some(value.into());
        self
    }

    // NOTE: Include `interestfor` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#interestfor)

    /// URLs to ping.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#ping)
    fn ping(mut self, value: impl Into<Bake>) -> Self {
        self.area_attrs_mut().ping = Some(value.into());
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#referrerpolicy)
    fn referrerpolicy(mut self, value: impl Into<Bake>) -> Self {
        self.area_attrs_mut().referrerpolicy = Some(value.into());
        self
    }

    /// Relationship between the location in the document containing the
    /// hyperlink and the destination resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
    fn rel(mut self, value: impl Into<Bake>) -> Self {
        self.area_attrs_mut().rel = Some(value.into());
        self
    }

    /// The kind of shape to be created in an image map.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#shape)
    fn shape(mut self, value: impl Into<Bake>) -> Self {
        self.area_attrs_mut().shape = Some(value.into());
        self
    }

    /// Navigable for hyperlink navigation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/area#target)
    fn target(mut self, value: impl Into<Bake>) -> Self {
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

impl<R: AreaRecipe> HasAreaAttrs for HtmlArea<R> {
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
/// assert_eq!(area.bake(), r#"<area id="image_map_area" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let area = area!()
///     .href("https://w.wiki/LTnF")
///     .alt("Red triangle")
///     .shape("poly")
///     .coords("300,63,470,357,130,357");
///
/// assert_eq!(
///     area.bake_pretty(),
///     r#"<area
///   shape="poly"
///   coords="300,63,470,357,130,357"
///   href="https://w.wiki/LTnF"
///   alt="Red triangle"
/// />
/// "#
/// );
/// ```
#[macro_export]
macro_rules! area {
    () => {
        $crate::html::HtmlArea::new()
    };
}
