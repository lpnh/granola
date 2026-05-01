use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// application
pub trait AudioTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlAudio<Self>) -> HtmlAudio<Self> {
        element
    }
}

impl AudioTag for () {}

/// The HTML `<audio>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let audio: HtmlAudio = HtmlAudio::empty().id("embed_audio");
///
/// assert_eq!(audio.bake(),
/// r#"<audio id="embed_audio"></audio>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let audio: HtmlAudio = HtmlAudio::from_src("toms-dinner.mp3").autoplay(true);
///
/// assert_eq!(audio.bake(),
/// r#"<audio src="toms-dinner.mp3" autoplay></audio>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <audio
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</audio>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlAudio<M: AudioTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: AudioTag> HtmlAudio<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }

    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        let element = Self::default().src(src);

        M::recipe(element)
    }

    /// Hint that the media resource can be started automatically when the page is loaded.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#autoplay)
    pub fn autoplay(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("autoplay");
        }
        self
    }

    /// Show user agent controls.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#controls)
    pub fn controls(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("controls");
        }
        self
    }

    // NOTE: Include `controlslist` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#controlslist)

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    pub fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("crossorigin", value);
        self
    }

    // NOTE: Include `disableremoteplayback` in the future.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#disableremoteplayback)

    /// Used when determining loading deferral.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#loading)
    pub fn loading(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("loading", value);
        self
    }

    /// Whether to loop the media resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#loop)
    pub fn loop_(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("loop");
        }
        self
    }

    /// Whether to mute the media resource by default.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#muted)
    pub fn muted(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("muted");
        }
        self
    }

    /// Hints how much buffering the media resource will likely need.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#preload)
    pub fn preload(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("preload", value);
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }
}

/// Shorthand for `HtmlAudio<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let audio = audio!().id("embed_audio");
///
/// assert_eq!(audio.bake(),
/// r#"<audio id="embed_audio"></audio>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let audio = audio!(@from_src "toms-dinner.mp3").autoplay(true);
///
/// assert_eq!(audio.bake(),
/// r#"<audio src="toms-dinner.mp3" autoplay></audio>"#);
/// ```
#[macro_export]
macro_rules! audio {
    () => {
        $crate::html::HtmlAudio::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlAudio::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlAudio::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlAudio::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlAudio::<()>::new($crate::bake_inline![$($content),+])
    };
    (@from_src $src: expr $(,)?) => {
        $crate::html::HtmlAudio::<()>::from_src($src)
    };
}
