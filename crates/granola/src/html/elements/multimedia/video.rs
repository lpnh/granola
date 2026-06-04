use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<video>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let video = HtmlVideo::new().id("video_embed");
///
/// assert_eq!(video.bake(), r#"<video id="video_embed"></video>"#);
/// ```
/// ```rust
/// use granola::prelude::*;
///
/// let video = HtmlVideo::new()
///     .src("Never_Gonna_Give_You_Up.mp4")
///     .width(800)
///     .height(600)
///     .autoplay(true);
///
/// assert_eq!(
///     video.bake(),
///     r#"<video src="Never_Gonna_Give_You_Up.mp4" width="800" height="600" autoplay></video>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <video
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</video>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = VideoRecipe, content = Cow<'static, str>)]
pub struct HtmlVideo<R: VideoRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// application
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: VideoAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlVideo {
    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        Self::new().src(src)
    }
}

/// The HTML `<video>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- src | bake_attr("src") -}}
/// {{- poster | bake_attr("poster") -}}
/// {{- width | bake_attr("width") -}}
/// {{- height | bake_attr("height") -}}
/// {{- crossorigin | bake_attr("crossorigin") -}}
/// {{- preload | bake_attr("preload") -}}
/// {{- autoplay | bake_bool_attr("autoplay") -}}
/// {{- controls | bake_bool_attr("controls") -}}
/// {{- media_loop | bake_bool_attr("loop") -}}
/// {{- muted | bake_bool_attr("muted") -}}
/// {{- playsinline | bake_bool_attr("playsinline") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct VideoAttrs {
    pub src: Option<Cow<'static, str>>,
    pub poster: Option<Cow<'static, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub crossorigin: Option<Cow<'static, str>>,
    pub preload: Option<Cow<'static, str>>,
    pub autoplay: bool,
    pub controls: bool,
    pub media_loop: bool,
    pub muted: bool,
    pub playsinline: bool,
}

pub trait HasVideoAttrs: Sized {
    fn video_attrs_mut(&mut self) -> &mut VideoAttrs;

    /// Hint that the media resource can be started automatically when the page
    /// is loaded.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#autoplay)
    fn autoplay(mut self, value: bool) -> Self {
        self.video_attrs_mut().autoplay = value;
        self
    }

    /// Show user agent controls.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#controls)
    fn controls(mut self, value: bool) -> Self {
        self.video_attrs_mut().controls = value;
        self
    }

    // NOTE: Include `controlslist` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#controlslist)

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.video_attrs_mut().crossorigin = Some(value.into());
        self
    }

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#height)
    fn height(mut self, value: u32) -> Self {
        self.video_attrs_mut().height = Some(value);
        self
    }

    // NOTE: Include `loading` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#loading)

    /// Whether to loop the media resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#loop)
    fn media_loop(mut self, value: bool) -> Self {
        self.video_attrs_mut().media_loop = value;
        self
    }

    /// Whether to mute the media resource by default.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#muted)
    fn muted(mut self, value: bool) -> Self {
        self.video_attrs_mut().muted = value;
        self
    }

    /// Encourage the user agent to display video content within the element's
    /// playback area.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#playsinline)
    fn playsinline(mut self, value: bool) -> Self {
        self.video_attrs_mut().playsinline = value;
        self
    }

    /// Poster frame to show prior to video playback.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#poster)
    fn poster(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.video_attrs_mut().poster = Some(value.into());
        self
    }

    /// Hints how much buffering the media resource will likely need.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#preload)
    fn preload(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.video_attrs_mut().preload = Some(value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#src)
    fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.video_attrs_mut().src = Some(value.into());
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#width)
    fn width(mut self, value: u32) -> Self {
        self.video_attrs_mut().width = Some(value);
        self
    }
}

impl HasVideoAttrs for VideoAttrs {
    fn video_attrs_mut(&mut self) -> &mut VideoAttrs {
        self
    }
}

impl HasVideoAttrs for &mut VideoAttrs {
    fn video_attrs_mut(&mut self) -> &mut VideoAttrs {
        self
    }
}

impl<R: VideoRecipe> HasVideoAttrs for HtmlVideo<R> {
    fn video_attrs_mut(&mut self) -> &mut VideoAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlVideo`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let video = video!().id("video_embed");
///
/// assert_eq!(video.bake(), r#"<video id="video_embed"></video>"#);
/// ```
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let video = video!(@from_src "Never_Gonna_Give_You_Up.mp4")
///     .width(800)
///     .height(600)
///     .autoplay(true);
///
/// assert_eq!(video.bake(),
/// r#"<video src="Never_Gonna_Give_You_Up.mp4" width="800" height="600" autoplay></video>"#);
/// ```
#[macro_export]
macro_rules! video {
    () => {
        $crate::html::HtmlVideo::new()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlVideo::new().content($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlVideo::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@from_src $src: expr $(,)?) => {
        $crate::html::HtmlVideo::from_src($src)
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlVideo::new().content($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlVideo::new().content($crate::bake_inline![$($content),+])
    };
}
