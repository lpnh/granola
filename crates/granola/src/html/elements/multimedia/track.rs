use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait TrackTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
}

impl TrackTag for () {}

/// The HTML `<track />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let track: HtmlTrack = HtmlTrack::empty().id("embed_text_track");
///
/// assert_eq!(track.bake(),
/// r#"<track id="embed_text_track" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let track: HtmlTrack = HtmlTrack::new("der_himmel_uber_berlin.vtt")
///     .kind("captions")
///     .enabled(true);
///
/// assert_eq!(track.bake(),
/// r#"<track src="der_himmel_uber_berlin.vtt" kind="captions" default />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <track
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTrack<M: TrackTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TrackTag> HtmlTrack<M> {
    pub fn new(src: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s.src(src)
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    /// Enable the track if no other text track is more suitable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#default)
    pub fn enabled(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("default");
        }
        self
    }

    /// The type of text track.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#kind)
    pub fn kind(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("kind", value);
        self
    }

    /// User-visible label.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#label)
    pub fn label(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("label", value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }

    /// Language of the text track.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#srclangrc)
    pub fn srclang(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("srclang", value);
        self
    }
}
