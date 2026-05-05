use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<img />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let img: HtmlImg = HtmlImg::empty().id("image_embed");
///
/// assert_eq!(img.bake(),
/// r#"<img id="image_embed" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let img: HtmlImg = HtmlImg::new("profile.png", "A mustachioed in a red cap and blue overalls");
///
/// assert_eq!(img.bake(),
/// r#"<img src="profile.png" alt="A mustachioed in a red cap and blue overalls" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <img
///   {{- attrs -}}
///   {{- specific_attrs }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ImgTag, specific = ImgAttrs)]
pub struct HtmlImg<M: ImgTag = ()> {
    _marker: PhantomData<M>,
    /// # Permitted ARIA roles
    ///
    /// with non-empty alt attribute:
    ///   button, checkbox, link, menuitem, menuitemcheckbox, menuitemradio, option,
    ///   progressbar, scrollbar, separator, slider, switch, tab, treeitem
    /// with empty alt attribute: none or presentation
    /// with no alt attribute: no role permitted
    pub attrs: Attrs,
    pub specific_attrs: ImgAttrs,
}

impl<M: ImgTag> HtmlImg<M> {
    pub fn new(src: impl Into<Cow<'static, str>>, alt: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = ImgAttrs::default().src(src).alt(alt);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }

    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = ImgAttrs::default().src(src);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }
}

/// The HTML `<todo>` element specific attributes.
///
/// [MDN Documentation]()
///
/// # Askama template
///
/// ```askama
/// {{- alt | bake_attr("alt") -}}
/// {{- crossorigin | bake_attr("crossorigin") -}}
/// {{- decoding | bake_attr("decoding") -}}
/// {{- elementtiming | bake_attr("elementtiming") -}}
/// {{- fetchpriority | bake_attr("fetchpriority") -}}
/// {{- height | bake_attr("height") -}}
/// {{- ismap | bake_bool_attr("ismap") -}}
/// {{- loading | bake_attr("loading") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- sizes | bake_attr("sizes") -}}
/// {{- src | bake_attr("src") -}}
/// {{- srcset | bake_attr("srcset") -}}
/// {{- width | bake_attr("width") -}}
/// {{- usemap | bake_attr("usemap") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ImgAttrs {
    pub alt: Option<Cow<'static, str>>,
    pub crossorigin: Option<Cow<'static, str>>,
    pub decoding: Option<Cow<'static, str>>,
    pub elementtiming: Option<Cow<'static, str>>,
    pub fetchpriority: Option<Cow<'static, str>>,
    pub height: Option<u32>,
    pub ismap: bool,
    pub loading: Option<Cow<'static, str>>,
    pub referrerpolicy: Option<Cow<'static, str>>,
    pub sizes: Option<Cow<'static, str>>,
    pub src: Option<Cow<'static, str>>,
    pub srcset: Option<Cow<'static, str>>,
    pub width: Option<u32>,
    pub usemap: Option<Cow<'static, str>>,
}

pub trait HasImgAttrs: Sized {
    fn img_attrs_mut(&mut self) -> &mut ImgAttrs;

    /// Replacement text for use when images are not available.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#alt)
    fn alt(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().alt = Some(value.into());
        self
    }

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().crossorigin = Some(value.into());
        self
    }

    /// Decoding hint to use when processing this image for presentation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#decoding)
    fn decoding(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().decoding = Some(value.into());
        self
    }

    /// Marks the image for observation by the `PerformanceElementTiming` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/elementtiming)
    fn elementtiming(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().elementtiming = Some(value.into());
        self
    }

    /// Sets the priority for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/fetchpriority)
    fn fetchpriority(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().fetchpriority = Some(value.into());
        self
    }

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#height)
    fn height(mut self, value: u32) -> Self {
        self.img_attrs_mut().height = Some(value);
        self
    }

    /// Whether the image is a server-side image map.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#ismap)
    fn ismap(mut self, value: bool) -> Self {
        self.img_attrs_mut().ismap = value;
        self
    }

    /// Used when determining loading deferral.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#loading)
    fn loading(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().loading = Some(value.into());
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#referrerpolicy)
    fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().referrerpolicy = Some(value.into());
        self
    }

    /// Image sizes for different page layouts.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#sizes)
    fn sizes(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().sizes = Some(value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#src)
    fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().src = Some(value.into());
        self
    }

    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#srcset)
    fn srcset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().srcset = Some(value.into());
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#width)
    fn width(mut self, value: u32) -> Self {
        self.img_attrs_mut().width = Some(value);
        self
    }

    /// Name of image map to use.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#usemap)
    fn usemap(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.img_attrs_mut().usemap = Some(value.into());
        self
    }
}

impl HasImgAttrs for ImgAttrs {
    fn img_attrs_mut(&mut self) -> &mut ImgAttrs {
        self
    }
}

impl HasImgAttrs for &mut ImgAttrs {
    fn img_attrs_mut(&mut self) -> &mut ImgAttrs {
        self
    }
}

impl<M: ImgTag> HasImgAttrs for HtmlImg<M> {
    fn img_attrs_mut(&mut self) -> &mut ImgAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlImg`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let img = img!().id("image_embed");
///
/// assert_eq!(img.bake(),
/// r#"<img id="image_embed" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let img = img!("profile.png", "A mustachioed in a red cap and blue overalls");
///
/// assert_eq!(img.bake(),
/// r#"<img src="profile.png" alt="A mustachioed in a red cap and blue overalls" />"#);
/// ```
#[macro_export]
macro_rules! img {
    () => {
        $crate::html::HtmlImg::<()>::empty()
    };
    ($src: expr, $alt: expr $(,)?) => {
        $crate::html::HtmlImg::<()>::new($src, $alt)
    };
    (@from_src $src: expr $(,)?) => {
        $crate::html::HtmlImg::<()>::from_src($src)
    };
}
