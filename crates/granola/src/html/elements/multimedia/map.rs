use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<map>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/map)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let map: HtmlMap = HtmlMap::empty().id("image_map");
///
/// assert_eq!(map.bake(),
/// r#"<map id="image_map"></map>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let img: HtmlImg = HtmlImg::new("mg_flag.png", "MG flag")
///     .width(600)
///     .height(420)
///     .usemap("#minas-gerais");
///
/// let area: HtmlArea = HtmlArea::new("https://w.wiki/LTnF", "Red triangle")
///     .shape("poly")
///     .coords("300,63,470,357,130,357");
///
/// let map: HtmlMap = HtmlMap::new(area).name("minas-gerais");
///
/// let img_map = bake_block![img, map];
///
/// assert_eq!(img_map,
/// r##"<img src="mg_flag.png" alt="MG flag" width="600" height="420" usemap="#minas-gerais" />
/// <map name="minas-gerais">
///   <area href="https://w.wiki/LTnF" alt="Red triangle" shape="poly" coords="300,63,470,357,130,357" />
/// </map>"##);
/// ```
///
/// # Askama template
///
/// ```askama
/// <map
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</map>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MapTag, content = Areas, specific = MapAttrs)]
pub struct HtmlMap<M: MapTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub attrs: Attrs,
    pub specific_attrs: MapAttrs,
}

/// The HTML `<map>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/map#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- name | bake_attr("name") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct MapAttrs {
    pub name: Option<Cow<'static, str>>,
}

pub trait HasMapAttrs: Sized {
    fn map_attrs_mut(&mut self) -> &mut MapAttrs;

    /// Name of image map to reference from the usemap attribute.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/map#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.map_attrs_mut().name = Some(value.into());
        self
    }
}

impl HasMapAttrs for MapAttrs {
    fn map_attrs_mut(&mut self) -> &mut MapAttrs {
        self
    }
}

impl HasMapAttrs for &mut MapAttrs {
    fn map_attrs_mut(&mut self) -> &mut MapAttrs {
        self
    }
}

impl<M: MapTag> HasMapAttrs for HtmlMap<M> {
    fn map_attrs_mut(&mut self) -> &mut MapAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlMap`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let map = map!().id("image_map");
///
/// assert_eq!(map.bake(),
/// r#"<map id="image_map"></map>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let img = img!("mg_flag.png", "MG flag")
///     .width(600)
///     .height(420)
///     .usemap("#minas-gerais");
///
/// let area = area!("https://w.wiki/LTnF", "Red triangle")
///     .shape("poly")
///     .coords("300,63,470,357,130,357");
///
/// let map = map!(area).name("minas-gerais");
///
/// let img_map = bake_block![img, map];
///
/// assert_eq!(img_map,
/// r##"<img src="mg_flag.png" alt="MG flag" width="600" height="420" usemap="#minas-gerais" />
/// <map name="minas-gerais">
///   <area href="https://w.wiki/LTnF" alt="Red triangle" shape="poly" coords="300,63,470,357,130,357" />
/// </map>"##);
/// ```
#[macro_export]
macro_rules! map {
    () => {
        $crate::html::HtmlMap::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlMap::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlMap::<()>::new([$first $(, $rest)*])
    };
}
