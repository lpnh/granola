use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<link />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let link = HtmlLink::new().id("external_resource_link");
///
/// assert_eq!(link.bake(), r#"<link id="external_resource_link" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let link = HtmlLink::new().href("fancy.css").rel("stylesheet");
///
/// assert_eq!(link.bake(), r#"<link href="fancy.css" rel="stylesheet" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <link
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = LinkRecipe)]
pub struct HtmlLink<R: LinkRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: LinkAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<link />` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- preload_as | bake_attr("as") -}}
/// {{- blocking | bake_attr("blocking") -}}
/// {{- crossorigin | bake_attr("crossorigin") -}}
/// {{- fetchpriority | bake_attr("fetchpriority") -}}
/// {{- href | bake_attr("href") -}}
/// {{- hreflang | bake_attr("hreflang") -}}
/// {{- imagesizes | bake_attr("imagesizes") -}}
/// {{- imagesrcset | bake_attr("imagesrcset") -}}
/// {{- integrity | bake_attr("integrity") -}}
/// {{- media | bake_attr("media") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- rel | bake_attr("rel") -}}
/// {{- sizes | bake_attr("sizes") -}}
/// {{- mime_type | bake_attr("type") -}}
/// {{- disabled | bake_bool_attr("defer") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct LinkAttrs {
    pub preload_as: Option<Cow<'static, str>>,
    pub blocking: Option<Cow<'static, str>>,
    pub crossorigin: Option<Cow<'static, str>>,
    pub fetchpriority: Option<Cow<'static, str>>,
    pub href: Option<Cow<'static, str>>,
    pub hreflang: Option<Cow<'static, str>>,
    pub imagesizes: Option<Cow<'static, str>>,
    pub imagesrcset: Option<Cow<'static, str>>,
    pub integrity: Option<Cow<'static, str>>,
    pub media: Option<Cow<'static, str>>,
    pub referrerpolicy: Option<Cow<'static, str>>,
    pub rel: Option<Cow<'static, str>>,
    pub sizes: Option<Cow<'static, str>>,
    pub mime_type: Option<Cow<'static, str>>,
    pub disabled: bool,
}

impl HtmlLink {
    pub fn from_href_rel(
        href: impl Into<Cow<'static, str>>,
        rel: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::from_href(href).rel(rel)
    }

    pub fn from_href(href: impl Into<Cow<'static, str>>) -> Self {
        Self::new().href(href)
    }
}

pub trait HasLinkAttrs: Sized {
    fn link_attrs_mut(&mut self) -> &mut LinkAttrs;

    /// Destination for a preload request (for `rel="preload"` and
    /// `rel="modulepreload"`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#as)
    fn preload_as(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().preload_as = Some(value.into());
        self
    }

    /// Whether the element is potentially render-blocking.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#blocking)
    fn blocking(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().blocking = Some(value.into());
        self
    }

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().crossorigin = Some(value.into());
        self
    }

    /// Whether the link is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#disabled)
    fn disabled(mut self, value: bool) -> Self {
        self.link_attrs_mut().disabled = value;
        self
    }

    /// Sets the priority for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/fetchpriority)
    fn fetchpriority(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().fetchpriority = Some(value.into());
        self
    }

    /// Address of the hyperlink.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#href)
    fn href(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().href = Some(value.into());
        self
    }

    /// Language of the linked resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#hreflang)
    fn hreflang(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().hreflang = Some(value.into());
        self
    }

    /// Image sizes for different page layouts (for `rel="preload"`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#imagesizes)
    fn imagesizes(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().imagesizes = Some(value.into());
        self
    }

    /// Images to use in different situations, e.g., high-resolution displays,
    /// small monitors, etc. (for `rel="preload"`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#imagesrcset)
    fn imagesrcset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().imagesrcset = Some(value.into());
        self
    }

    /// Integrity metadata used in Subresource Integrity checks.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#integrity)
    fn integrity(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().integrity = Some(value.into());
        self
    }

    /// Applicable media.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#media)
    fn media(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().media = Some(value.into());
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#referrerpolicy)
    fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().referrerpolicy = Some(value.into());
        self
    }

    /// Relationship between the location in the document containing the
    /// hyperlink and the destination resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
    fn rel(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().rel = Some(value.into());
        self
    }

    /// Sizes of the icons (for `rel="icon"`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#sizes)
    fn sizes(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().sizes = Some(value.into());
        self
    }

    /// Hint for the type of the referenced resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#type)
    ///
    /// See [`MimeType`]
    fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.link_attrs_mut().mime_type = Some(value.into());
        self
    }
}

impl HasLinkAttrs for LinkAttrs {
    fn link_attrs_mut(&mut self) -> &mut LinkAttrs {
        self
    }
}

impl HasLinkAttrs for &mut LinkAttrs {
    fn link_attrs_mut(&mut self) -> &mut LinkAttrs {
        self
    }
}

impl<R: LinkRecipe> HasLinkAttrs for HtmlLink<R> {
    fn link_attrs_mut(&mut self) -> &mut LinkAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlLink`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let link = link!().id("external_resource_link");
///
/// assert_eq!(link.bake(), r#"<link id="external_resource_link" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let link = link!(@href_rel "fancy.css", "stylesheet");
///
/// assert_eq!(link.bake(), r#"<link href="fancy.css" rel="stylesheet" />"#);
/// ```
#[macro_export]
macro_rules! link {
    () => {
        $crate::html::HtmlLink::new()
    };

    (@href_rel $href:expr, $rel:expr $(,)?) => {
        $crate::html::HtmlLink::from_href_rel($href, $rel)
    };
    (@href $href:expr $(,)?) => {
        $crate::html::HtmlLink::from_href($href)
    };
    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlLink::<$r>::from_cookbook()
    };
}
