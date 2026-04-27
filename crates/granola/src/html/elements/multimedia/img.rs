use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait ImgTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles:
    ///     - with non-empty alt attribute:
    ///         button, checkbox, link, menuitem, menuitemcheckbox, menuitemradio, option,
    ///         progressbar, scrollbar, separator, slider, switch, tab, treeitem
    ///     - with empty alt attribute: none or presentation
    ///     - with no alt attribute: no role permitted
    const ROLE: Option<&'static str> = None;
}

impl ImgTag for () {}

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
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlImg<M: ImgTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ImgTag> HtmlImg<M> {
    pub fn new(src: impl Into<Cow<'static, str>>, alt: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s.src(src).alt(alt)
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

    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s.src(src)
    }

    /// Replacement text for use when images are not available.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#alt)
    pub fn alt(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("alt", value);
        self
    }

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    pub fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("crossorigin", value);
        self
    }

    /// Decoding hint to use when processing this image for presentation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#decoding)
    pub fn decoding(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("decoding", value);
        self
    }

    /// Marks the image for observation by the `PerformanceElementTiming` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/elementtiming)
    pub fn elementtiming(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("elementtiming", value);
        self
    }

    /// Sets the priority for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/fetchpriority)
    pub fn fetchpriority(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("fetchpriority", value);
        self
    }

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#height)
    pub fn height(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("height", value.to_string());
        self
    }

    /// Whether the image is a server-side image map.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#ismap)
    pub fn ismap(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("ismap");
        }
        self
    }

    /// Used when determining loading deferral.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#loading)
    pub fn loading(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("loading", value);
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#referrerpolicy)
    pub fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("referrerpolicy", value);
        self
    }

    /// Image sizes for different page layouts.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#sizes)
    pub fn sizes(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("sizes", value);
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }

    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#srcset)
    pub fn srcset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("srcset", value);
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#width)
    pub fn width(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("width", value.to_string());
        self
    }

    /// Name of image map to use.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#usemap)
    pub fn usemap(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("usemap", value);
        self
    }
}

/// Shorthand for `HtmlImg<()>`.
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
