use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait VideoTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: application
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl VideoTag for () {}

/// The HTML `<video>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let video: HtmlVideo = HtmlVideo::empty().id("video_embed");
///
/// assert_eq!(video.bake(),
/// r#"<video id="video_embed"></video>"#);
/// ```
/// ```rust
/// use granola::prelude::*;
///
/// let video: HtmlVideo = HtmlVideo::from_src("Never_Gonna_Give_You_Up.mp4")
///     .width(800)
///     .height(600)
///     .autoplay(true);
///
/// assert_eq!(video.bake(),
/// r#"<video src="Never_Gonna_Give_You_Up.mp4" width="800" height="600" autoplay></video>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <video
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</video>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlVideo<M: VideoTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: VideoTag> HtmlVideo<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
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

    /// Hint that the media resource can be started automatically when the page is loaded.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#autoplay)
    pub fn autoplay(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("autoplay");
        }
        self
    }

    /// Show user agent controls.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#controls)
    pub fn controls(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("controls");
        }
        self
    }

    // NOTE: Include `controlslist` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#controlslist)

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    pub fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("crossorigin", value);
        self
    }
    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#height)
    pub fn height(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("height", value.to_string());
        self
    }

    // NOTE: Include `loading` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#loading)

    /// Whether to loop the media resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#loop)
    pub fn loop_(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("loop");
        }
        self
    }

    /// Whether to mute the media resource by default.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#muted)
    pub fn muted(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("muted");
        }
        self
    }

    /// Encourage the user agent to display video content within the element's playback area.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#playsinline)
    pub fn playsinline(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("playsinline");
        }
        self
    }

    /// Poster frame to show prior to video playback.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#poster)
    pub fn poster(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("poster", value);
        self
    }

    /// Hints how much buffering the media resource will likely need.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#preload)
    pub fn preload(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("preload", value);
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/video#width)
    pub fn width(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("width", value.to_string());
        self
    }
}

/// Shorthand for `HtmlVideo<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let video = video!().id("video_embed");
///
/// assert_eq!(video.bake(),
/// r#"<video id="video_embed"></video>"#);
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
        $crate::html::HtmlVideo::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlVideo::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlVideo::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlVideo::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlVideo::<()>::new($crate::bake_inline![$($content),+])
    };
    (@from_src $src: expr $(,)?) => {
        $crate::html::HtmlVideo::<()>::from_src($src)
    };
}
