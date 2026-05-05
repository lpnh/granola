use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// let iframe: HtmlIframe = HtmlIframe::from_src("https://w.wiki/LJK7")
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
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</iframe>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = IframeTag, content = Cow<'static, str>, specific = IframeAttrs)]
pub struct HtmlIframe<M: IframeTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// application, document, img, none, presentation
    pub attrs: Attrs,
    pub specific_attrs: IframeAttrs,
}

impl<M: IframeTag> HtmlIframe<M> {
    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = IframeAttrs::default().src(src);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }
}

/// The HTML `<iframe>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- allow | bake_attr("allow") -}}
/// {{- allowfullscreen | bake_bool_attr("allowfullscreen") -}}
/// {{- height | bake_attr("height") -}}
/// {{- loading | bake_attr("loading") -}}
/// {{- name | bake_attr("name") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- sandbox | bake_attr("sandbox") -}}
/// {{- src | bake_attr("src") -}}
/// {{- srcdoc | bake_attr("srcdoc") -}}
/// {{- width | bake_attr("width") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct IframeAttrs {
    pub allow: Option<Cow<'static, str>>,
    pub allowfullscreen: bool,
    pub height: Option<u32>,
    pub loading: Option<Cow<'static, str>>,
    pub name: Option<Cow<'static, str>>,
    pub referrerpolicy: Option<Cow<'static, str>>,
    pub sandbox: Option<Cow<'static, str>>,
    pub src: Option<Cow<'static, str>>,
    pub srcdoc: Option<Cow<'static, str>>,
    pub width: Option<u32>,
}

pub trait HasIframeAttrs: Sized {
    fn iframe_attrs_mut(&mut self) -> &mut IframeAttrs;

    /// Permissions policy to be applied to the iframe's contents.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#allow)
    fn allow(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.iframe_attrs_mut().allow = Some(value.into());
        self
    }

    /// Whether to allow the iframe's contents to use `requestFullscreen()`.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#allowfullscreen)
    fn allowfullscreen(mut self, value: bool) -> Self {
        self.iframe_attrs_mut().allowfullscreen = value;
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
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#height)
    fn height(mut self, value: u32) -> Self {
        self.iframe_attrs_mut().height = Some(value);
        self
    }

    /// Used when determining loading deferral.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#loading)
    fn loading(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.iframe_attrs_mut().loading = Some(value.into());
        self
    }

    /// Name of content navigable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.iframe_attrs_mut().name = Some(value.into());
        self
    }

    // NOTE: Include `privateToken` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#privatetoken)

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#referrerpolicy)
    fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.iframe_attrs_mut().referrerpolicy = Some(value.into());
        self
    }

    /// Security rules for nested content.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#sandbox)
    fn sandbox(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.iframe_attrs_mut().sandbox = Some(value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#src)
    fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.iframe_attrs_mut().src = Some(value.into());
        self
    }

    /// A document to render in the iframe.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#srcdoc)
    fn srcdoc(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.iframe_attrs_mut().srcdoc = Some(value.into());
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#width)
    fn width(mut self, value: u32) -> Self {
        self.iframe_attrs_mut().width = Some(value);
        self
    }
}

impl HasIframeAttrs for IframeAttrs {
    fn iframe_attrs_mut(&mut self) -> &mut IframeAttrs {
        self
    }
}

impl HasIframeAttrs for &mut IframeAttrs {
    fn iframe_attrs_mut(&mut self) -> &mut IframeAttrs {
        self
    }
}

impl<M: IframeTag> HasIframeAttrs for HtmlIframe<M> {
    fn iframe_attrs_mut(&mut self) -> &mut IframeAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlIframe`.
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
/// let iframe = iframe!(@from_src "https://w.wiki/LJK7")
///     .title("Pedestrians crossing an intersection.");
///
/// assert_eq!(iframe.bake(),
/// r#"<iframe title="Pedestrians crossing an intersection." src="https://w.wiki/LJK7"></iframe>"#);
/// ```
#[macro_export]
macro_rules! iframe {
    () => {
        $crate::html::HtmlIframe::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlIframe::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlIframe::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlIframe::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlIframe::<()>::new($crate::bake_inline![$($content),+])
    };

    (@from_src $src: expr $(,)?) => {
        $crate::html::HtmlIframe::<()>::from_src($src)
    };
}
