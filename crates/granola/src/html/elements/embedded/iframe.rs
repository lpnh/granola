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
/// let iframe = HtmlIframe::new().id("inline_frame");
///
/// assert_eq!(iframe.bake(), r#"<iframe id="inline_frame"></iframe>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let iframe = HtmlIframe::new()
///     .src("https://w.wiki/LJK7")
///     .title("Pedestrians crossing an intersection.");
///
/// assert_eq!(
///     iframe.bake(),
///     r#"<iframe title="Pedestrians crossing an intersection." src="https://w.wiki/LJK7"></iframe>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <iframe
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</iframe>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = IframeRecipe, content = Cow<'static, str>)]
pub struct HtmlIframe<R: IframeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// application, document, img, none, presentation
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: IframeAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlIframe {
    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        Self::new().src(src)
    }
}

/// The HTML `<iframe>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/iframe#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- src | bake_attr("src") -}}
/// {{- width | bake_attr("width") -}}
/// {{- height | bake_attr("height") -}}
/// {{- allow | bake_attr("allow") -}}
/// {{- loading | bake_attr("loading") -}}
/// {{- name | bake_attr("name") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- sandbox | bake_attr("sandbox") -}}
/// {{- srcdoc | bake_attr("srcdoc") -}}
/// {{- allowfullscreen | bake_bool_attr("allowfullscreen") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct IframeAttrs {
    pub src: Option<Cow<'static, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub allow: Option<Cow<'static, str>>,
    pub loading: Option<Cow<'static, str>>,
    pub name: Option<Cow<'static, str>>,
    pub referrerpolicy: Option<Cow<'static, str>>,
    pub sandbox: Option<Cow<'static, str>>,
    pub srcdoc: Option<Cow<'static, str>>,
    pub allowfullscreen: bool,
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

impl<R: IframeRecipe> HasIframeAttrs for HtmlIframe<R> {
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
/// assert_eq!(iframe.bake(), r#"<iframe id="inline_frame"></iframe>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let iframe = iframe!(@src "https://w.wiki/LJK7")
///     .title("Pedestrians crossing an intersection.");
///
/// assert_eq!(iframe.bake(),
/// r#"<iframe title="Pedestrians crossing an intersection." src="https://w.wiki/LJK7"></iframe>"#);
/// ```
#[macro_export]
macro_rules! iframe {
    () => {
        $crate::html::HtmlIframe::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlIframe::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlIframe::new().content($crate::bake![$first $(, $rest)*])
    };

    (@src $src:expr $(,)?) => {
        $crate::html::HtmlIframe::from_src($src)
    };

    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlIframe::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; $content:expr $(,)?) => {
        $crate::html::HtmlIframe::<$r>::from_cookbook().content($content)
    };
    (@cookbook $r:ty ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlIframe::<$r>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
