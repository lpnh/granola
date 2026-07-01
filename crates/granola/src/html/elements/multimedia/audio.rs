use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<audio>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let audio = HtmlAudio::new().id("embed_audio");
///
/// assert_eq!(audio.bake(), r#"<audio id="embed_audio"></audio>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let audio = HtmlAudio::new().src("toms-dinner.mp3").autoplay(true);
///
/// assert_eq!(
///     audio.bake(),
///     r#"<audio src="toms-dinner.mp3" autoplay></audio>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <audio
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</audio>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = AudioRecipe, content = Bake)]
pub struct HtmlAudio<R: AudioRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// application
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: AudioAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlAudio {
    pub fn from_src(src: impl Into<Cow<'static, str>>) -> Self {
        Self::new().src(src)
    }
}

/// The HTML `<audio>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- crossorigin | bake_attr("crossorigin") -}}
/// {{- preload | bake_attr("preload") -}}
/// {{- src | bake_attr("src") -}}
/// {{- autoplay | bake_bool_attr("autoplay") -}}
/// {{- controls | bake_bool_attr("controls") -}}
/// {{- media_loop | bake_bool_attr("loop") -}}
/// {{- muted | bake_bool_attr("muted") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct AudioAttrs {
    pub crossorigin: Option<Cow<'static, str>>,
    pub preload: Option<Cow<'static, str>>,
    pub src: Option<Cow<'static, str>>,
    pub autoplay: bool,
    pub controls: bool,
    pub media_loop: bool,
    pub muted: bool,
}

pub trait HasAudioAttrs: Sized {
    fn audio_attrs_mut(&mut self) -> &mut AudioAttrs;

    /// Hint that the media resource can be started automatically when the page
    /// is loaded.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#autoplay)
    fn autoplay(mut self, value: bool) -> Self {
        self.audio_attrs_mut().autoplay = value;
        self
    }

    /// Show user agent controls.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#controls)
    fn controls(mut self, value: bool) -> Self {
        self.audio_attrs_mut().controls = value;
        self
    }

    // NOTE: Include `controlslist` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#controlslist)

    /// How the element handles crossorigin requests.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/crossorigin)
    fn crossorigin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.audio_attrs_mut().crossorigin = Some(value.into());
        self
    }

    // NOTE: Include `disableremoteplayback` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#disableremoteplayback)

    // NOTE: Include `loading` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#loading)

    /// Whether to loop the media resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#loop)
    fn media_loop(mut self, value: bool) -> Self {
        self.audio_attrs_mut().media_loop = value;
        self
    }

    /// Whether to mute the media resource by default.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#muted)
    fn muted(mut self, value: bool) -> Self {
        self.audio_attrs_mut().muted = value;
        self
    }

    /// Hints how much buffering the media resource will likely need.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#preload)
    fn preload(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.audio_attrs_mut().preload = Some(value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/audio#src)
    fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.audio_attrs_mut().src = Some(value.into());
        self
    }
}

impl HasAudioAttrs for AudioAttrs {
    fn audio_attrs_mut(&mut self) -> &mut AudioAttrs {
        self
    }
}

impl HasAudioAttrs for &mut AudioAttrs {
    fn audio_attrs_mut(&mut self) -> &mut AudioAttrs {
        self
    }
}

impl<R: AudioRecipe> HasAudioAttrs for HtmlAudio<R> {
    fn audio_attrs_mut(&mut self) -> &mut AudioAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlAudio`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let audio = audio!().id("embed_audio");
///
/// assert_eq!(audio.bake(), r#"<audio id="embed_audio"></audio>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let audio = audio!().src("toms-dinner.mp3").autoplay(true);
///
/// assert_eq!(
///     audio.bake(),
///     r#"<audio src="toms-dinner.mp3" autoplay></audio>"#
/// );
/// ```
#[macro_export]
macro_rules! audio {
    () => {
        $crate::html::HtmlAudio::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlAudio::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlAudio::new().content($crate::bake![$first $(, $rest)*])
    };

}
