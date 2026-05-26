use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<picture>` element.
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
/// assert_eq!(picture.bake(), r#"<picture id="picture"></picture>"#);
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
/// assert_eq!(
///     picture.bake(),
///     r#"<picture>
///   <source srcset="logo-wide.png" media="(width >= 600px)" />
///   <img src="logo-narrow.png" alt="logo" />
/// </picture>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <picture
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</picture>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = PictureRecipe, content = Cow<'static, str>)]
pub struct HtmlPicture<R: PictureRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlPicture`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let picture = picture!().id("picture");
///
/// assert_eq!(picture.bake(), r#"<picture id="picture"></picture>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let source = source!().srcset("logo-wide.png").media("(width >= 600px)");
/// let img: HtmlImg = HtmlImg::new("logo-narrow.png", "logo");
///
/// let picture = picture!(source, img);
///
/// assert_eq!(
///     picture.bake(),
///     r#"<picture>
///   <source srcset="logo-wide.png" media="(width >= 600px)" />
///   <img src="logo-narrow.png" alt="logo" />
/// </picture>"#
/// );
/// ```
#[macro_export]
macro_rules! picture {
    () => {
        $crate::html::HtmlPicture::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlPicture::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlPicture::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlPicture::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlPicture::<()>::new($crate::bake_inline![$($content),+])
    };
}
