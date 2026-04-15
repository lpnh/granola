use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait PictureTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl PictureTag for () {}

/// The HTML `<picture>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/picture)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let picture: HtmlPicture = HtmlPicture::empty().id("picture");
///
/// assert_eq!(picture.bake(),
/// r#"<picture id="picture"></picture>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let source: HtmlSource = HtmlSource::empty()
///     .srcset("logo-wide.png")
///     .media("(width >= 600px)");
/// let img: HtmlImg = HtmlImg::new("logo-narrow.png", "logo");
///
/// let picture: HtmlPicture = HtmlPicture::new(bake_block![source, img]);
///
/// assert_eq!(picture.bake(),
/// r#"<picture>
///   <source srcset="logo-wide.png" media="(width >= 600px)" />
///   <img src="logo-narrow.png" alt="logo" />
/// </picture>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <picture
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2, 70) }}</picture>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlPicture<M: PictureTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: PictureTag> HtmlPicture<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }
}
