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
/// let img = HtmlImg::new().id("image_embed");
///
/// assert_eq!(img.bake(), r#"<img id="image_embed" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let img = HtmlImg::new()
///     .src("profile.png")
///     .alt("A mustachioed in a red cap and blue overalls");
///
/// assert_eq!(
///     img.bake(),
///     r#"<img src="profile.png" alt="A mustachioed in a red cap and blue overalls" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <img
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ImgRecipe)]
pub struct HtmlImg<R: ImgRecipe = ()> {
    _recipe: PhantomData<R>,
    /// # Permitted ARIA roles
    ///
    /// with non-empty alt attribute:
    ///   button, checkbox, link, menuitem, menuitemcheckbox, menuitemradio,
    /// option,   progressbar, scrollbar, separator, slider, switch, tab,
    /// treeitem with empty alt attribute: none or presentation
    /// with no alt attribute: no role permitted
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: ImgAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlImg {
    pub fn from_src_alt(
        src: impl Into<Cow<'static, str>>,
        alt: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::new().src(src).alt(alt)
    }

    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        Self::new().src(src)
    }
}

/// The HTML `<img>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/img#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- src | bake_attr("src") -}}
/// {{- alt | bake_attr("alt") -}}
/// {{- srcset | bake_attr("srcset") -}}
/// {{- sizes | bake_attr("sizes") -}}
/// {{- width | bake_attr("width") -}}
/// {{- height | bake_attr("height") -}}
/// {{- loading | bake_attr("loading") -}}
/// {{- decoding | bake_attr("decoding") -}}
/// {{- crossorigin | bake_attr("crossorigin") -}}
/// {{- elementtiming | bake_attr("elementtiming") -}}
/// {{- fetchpriority | bake_attr("fetchpriority") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- usemap | bake_attr("usemap") -}}
/// {{- ismap | bake_bool_attr("ismap") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ImgAttrs {
    pub src: Option<Cow<'static, str>>,
    pub alt: Option<Cow<'static, str>>,
    pub srcset: Option<Cow<'static, str>>,
    pub sizes: Option<Cow<'static, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub loading: Option<Cow<'static, str>>,
    pub decoding: Option<Cow<'static, str>>,
    pub crossorigin: Option<Cow<'static, str>>,
    pub elementtiming: Option<Cow<'static, str>>,
    pub fetchpriority: Option<Cow<'static, str>>,
    pub referrerpolicy: Option<Cow<'static, str>>,
    pub usemap: Option<Cow<'static, str>>,
    pub ismap: bool,
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

    /// Images to use in different situations, e.g., high-resolution displays,
    /// small monitors, etc.
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

impl<R: ImgRecipe> HasImgAttrs for HtmlImg<R> {
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
/// assert_eq!(img.bake(), r#"<img id="image_embed" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let img = img!(@src_alt
///     "profile.png",
///     "A mustachioed in a red cap and blue overalls"
/// );
///
/// assert_eq!(
///     img.bake(),
///     r#"<img src="profile.png" alt="A mustachioed in a red cap and blue overalls" />"#
/// );
/// ```
#[macro_export]
macro_rules! img {
    () => {
        $crate::html::HtmlImg::new()
    };

    (@src_alt $src:expr, $alt:expr $(,)?) => {
        $crate::html::HtmlImg::from_src_alt($src, $alt)
    };
    (@src $src:expr $(,)?) => {
        $crate::html::HtmlImg::from_src($src)
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlImg::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
}
