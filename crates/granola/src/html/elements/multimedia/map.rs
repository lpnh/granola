use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait MapTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Areas;
}

impl MapTag for () {}

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
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</map>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlMap<M: MapTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: MapTag> HtmlMap<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    /// Name of image map to reference from the usemap attribute.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/map#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }
}

/// Shorthand for `HtmlMap<()>`.
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
