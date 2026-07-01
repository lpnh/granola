use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<header>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/header)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let header = HtmlHeader::new().id("header");
///
/// assert_eq!(header.bake(), r#"<header id="header"></header>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let logo = HtmlA::new().content("Oats &amp; Ends").href("/");
///
/// let header = HtmlHeader::new().content(logo);
///
/// assert_eq!(
///     header.bake(),
///     r#"<header><a href="/">Oats &amp; Ends</a></header>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <header
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</header>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = HeaderRecipe, content = Bake)]
pub struct HtmlHeader<R: HeaderRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// group, presentation or none
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlHeader`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let header = header!().id("header");
///
/// assert_eq!(header.bake(), r#"<header id="header"></header>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let logo = a!("Oats &amp; Ends").href("/");
///
/// let header = header!(logo);
///
/// assert_eq!(
///     header.bake(),
///     r#"<header><a href="/">Oats &amp; Ends</a></header>"#
/// );
/// ```
#[macro_export]
macro_rules! header {
    () => {
        $crate::html::HtmlHeader::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlHeader::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlHeader::new().content($crate::bake![$first $(, $rest)*])
    };
}
