use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<track />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let track = HtmlTrack::new().id("embed_text_track");
///
/// assert_eq!(track.bake(), r#"<track id="embed_text_track" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let track = HtmlTrack::new()
///     .src("der_himmel_uber_berlin.vtt")
///     .kind("captions")
///     .enabled(true);
///
/// assert_eq!(
///     track.bake(),
///     r#"<track kind="captions" src="der_himmel_uber_berlin.vtt" default />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <track
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TrackRecipe)]
pub struct HtmlTrack<R: TrackRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: TrackAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlTrack {
    pub fn from_src(src: impl Into<Bake>) -> Self {
        Self::new().src(src)
    }
}

/// The HTML `<track />` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- kind | bake_attr("kind") -}}
/// {{- src | bake_attr("src") -}}
/// {{- srclang | bake_attr("srclang") -}}
/// {{- label | bake_attr("label") -}}
/// {{- enabled | bake_bool_attr("default") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TrackAttrs {
    pub kind: Option<Bake>,
    pub src: Option<Bake>,
    pub srclang: Option<Bake>,
    pub label: Option<Bake>,
    pub enabled: bool,
}

pub trait HasTrackAttrs: Sized {
    fn track_attrs_mut(&mut self) -> &mut TrackAttrs;

    /// Enable the track if no other text track is more suitable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#default)
    fn enabled(mut self, value: bool) -> Self {
        self.track_attrs_mut().enabled = value;
        self
    }

    /// The type of text track.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#kind)
    fn kind(mut self, value: impl Into<Bake>) -> Self {
        self.track_attrs_mut().kind = Some(value.into());
        self
    }

    /// User-visible label.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#label)
    fn label(mut self, value: impl Into<Bake>) -> Self {
        self.track_attrs_mut().label = Some(value.into());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#src)
    fn src(mut self, value: impl Into<Bake>) -> Self {
        self.track_attrs_mut().src = Some(value.into());
        self
    }

    /// Language of the text track.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/track#srclang)
    fn srclang(mut self, value: impl Into<Bake>) -> Self {
        self.track_attrs_mut().srclang = Some(value.into());
        self
    }
}

impl HasTrackAttrs for TrackAttrs {
    fn track_attrs_mut(&mut self) -> &mut TrackAttrs {
        self
    }
}

impl HasTrackAttrs for &mut TrackAttrs {
    fn track_attrs_mut(&mut self) -> &mut TrackAttrs {
        self
    }
}

impl<R: TrackRecipe> HasTrackAttrs for HtmlTrack<R> {
    fn track_attrs_mut(&mut self) -> &mut TrackAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlTrack`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let track = track!().id("embed_text_track");
///
/// assert_eq!(track.bake(), r#"<track id="embed_text_track" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let track = track!()
///     .src("der_himmel_uber_berlin.vtt")
///     .kind("captions")
///     .enabled(true);
///
/// assert_eq!(
///     track.bake(),
///     r#"<track kind="captions" src="der_himmel_uber_berlin.vtt" default />"#
/// );
/// ```
#[macro_export]
macro_rules! track {
    () => {
        $crate::html::HtmlTrack::new()
    };
}
