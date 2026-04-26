use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait LinkTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
}

impl LinkTag for () {}

/// The HTML `<link />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let link: HtmlLink = HtmlLink::empty().id("external_resource_link");
///
/// assert_eq!(link.bake(),
/// r#"<link id="external_resource_link" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let link: HtmlLink = HtmlLink::new("fancy.css", "stylesheet");
///
/// assert_eq!(link.bake(),
/// r#"<link href="fancy.css" rel="stylesheet" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <link
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlLink<M: LinkTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: LinkTag> HtmlLink<M> {
    pub fn new(href: impl Into<Cow<'static, str>>, rel: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s.href(href).rel(rel)
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    /// Destination for a preload request (for `rel="preload"` and `rel="modulepreload"`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#as)
    pub fn as_(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("as", value);
        self
    }

    /// Whether the element is potentially render-blocking.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#blocking)
    pub fn blocking(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("blocking", value);
        self
    }

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    pub fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("crossorigin", value);
        self
    }

    /// Whether the link is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#disabled)
    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("disabled");
        }
        self
    }

    /// Sets the priority for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/fetchpriority)
    pub fn fetchpriority(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("fetchpriority", value);
        self
    }

    /// Address of the hyperlink.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#href)
    pub fn href(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("href", value);
        self
    }

    /// Language of the linked resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#hreflang)
    pub fn hreflang(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("hreflang", value);
        self
    }

    /// Image sizes for different page layouts (for `rel="preload"`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#imagesizes)
    pub fn imagesizes(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("imagesizes", value);
        self
    }

    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc. (for `rel="preload"`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#imagesrcset)
    pub fn imagesrcset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("imagesrcset", value);
        self
    }

    /// Integrity metadata used in Subresource Integrity checks.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#integrity)
    pub fn integrity(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("integrity", value);
        self
    }

    /// Applicable media.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#media)
    pub fn media(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("media", value);
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#referrerpolicy)
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

    /// Sizes of the icons (for `rel="icon"`).
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#sizes)
    pub fn sizes(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("sizes", value);
        self
    }

    /// Hint for the type of the referenced resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/link#type)
    pub fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value);
        self
    }
}

/// Shorthand for `HtmlLink<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let link = link!().id("external_resource_link");
///
/// assert_eq!(link.bake(),
/// r#"<link id="external_resource_link" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let link = link!("fancy.css", "stylesheet");
///
/// assert_eq!(link.bake(),
/// r#"<link href="fancy.css" rel="stylesheet" />"#);
/// ```
#[macro_export]
macro_rules! link {
    () => {
        $crate::html::HtmlLink::<()>::empty()
    };
    ($href: expr, $rel: expr $(,)?) => {
        $crate::html::HtmlLink::<()>::new($href, $rel)
    };
}
