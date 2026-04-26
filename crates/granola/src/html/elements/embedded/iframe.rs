use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait IframeTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: application, document, img, none, presentation
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl IframeTag for () {}

/// The HTML `<iframe>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let iframe: HtmlIframe = HtmlIframe::empty().id("inline_frame");
///
/// assert_eq!(iframe.bake(),
/// r#"<iframe id="inline_frame"></iframe>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let iframe: HtmlIframe = HtmlIframe::new("https://w.wiki/LJK7")
///     .title("Pedestrians crossing an intersection.");
///
/// assert_eq!(iframe.bake(),
/// r#"<iframe title="Pedestrians crossing an intersection." src="https://w.wiki/LJK7"></iframe>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <iframe
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</iframe>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlIframe<M: IframeTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: IframeTag> HtmlIframe<M> {
    pub fn new(src: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s.src(src)
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

    /// Permissions policy to be applied to the iframe's contents.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#allow)
    pub fn allow(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("allow", value);
        self
    }

    /// Whether to allow the iframe's contents to use `requestFullscreen()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#allowfullscreen)
    pub fn allowfullscreen(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("allowfullscreen");
        }
        self
    }

    // NOTE: Include `credentialless` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#credentialless)

    // NOTE: Include `csp` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#csp)

    /// Vertical dimension.
    ///
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#height)
    pub fn height(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("height", value.to_string());
        self
    }

    /// Used when determining loading deferral.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#loading)
    pub fn loading(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("loading", value.into());
        self
    }

    /// Name of content navigable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value.into());
        self
    }

    // NOTE: Include `privateToken` in the future.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#privatetoken)

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#referrerpolicy)
    pub fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("referrerpolicy", value.into());
        self
    }

    /// Security rules for nested content.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#sandbox)
    pub fn sandbox(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("sandbox", value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }

    /// A document to render in the iframe.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#srcdoc)
    pub fn srcdoc(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("srcdoc", value);
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#width)
    pub fn width(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("width", value.to_string());
        self
    }
}

/// Shorthand for `HtmlIframe<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let iframe = iframe!().id("inline_frame");
///
/// assert_eq!(iframe.bake(),
/// r#"<iframe id="inline_frame"></iframe>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let iframe = iframe!("https://w.wiki/LJK7").title("Pedestrians crossing an intersection.");
///
/// assert_eq!(iframe.bake(),
/// r#"<iframe title="Pedestrians crossing an intersection." src="https://w.wiki/LJK7"></iframe>"#);
/// ```
#[macro_export]
macro_rules! iframe {
    () => {
        $crate::html::HtmlIframe::<()>::empty()
    };
    ($src: expr $(,)?) => {
        $crate::html::HtmlIframe::<()>::new($src)
    };
}
