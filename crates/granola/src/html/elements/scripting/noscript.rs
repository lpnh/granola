use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<noscript>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/noscript)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let noscript = HtmlNoscript::new().id("noscript");
///
/// assert_eq!(noscript.bake(), r#"<noscript id="noscript"></noscript>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let noscript = HtmlNoscript::new().content("It's javascript all the way down");
///
/// assert_eq!(
///     noscript.bake(),
///     r#"<noscript>It's javascript all the way down</noscript>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <noscript
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</noscript>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = NoscriptRecipe, content = Bake)]
pub struct HtmlNoscript<R: NoscriptRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlNoscript`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let noscript = noscript!().id("noscript");
///
/// assert_eq!(noscript.bake(), r#"<noscript id="noscript"></noscript>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let noscript = noscript!("It's javascript all the way down");
///
/// assert_eq!(
///     noscript.bake(),
///     r#"<noscript>It's javascript all the way down</noscript>"#
/// );
/// ```
#[macro_export]
macro_rules! noscript {
    () => {
        $crate::html::HtmlNoscript::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlNoscript::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlNoscript::new().content($crate::bake![$first $(, $rest)*])
    };

}
