use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<i>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/i)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let i = HtmlI::new().id("idiomatic_text");
///
/// assert_eq!(i.bake(), r#"<i id="idiomatic_text"></i>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let voila = HtmlI::new().content("voilà");
///
/// let quote = bake!["and ", voila, "!"];
///
/// assert_eq!(quote, r#"and <i>voilà</i>!"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <i
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</i>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = IRecipe, content = Bake)]
pub struct HtmlI<R: IRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlI`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let i = i!().id("idiomatic_text");
///
/// assert_eq!(i.bake(), r#"<i id="idiomatic_text"></i>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let voila = i!("voilà");
///
/// let quote = bake!["and ", voila, "!"];
///
/// assert_eq!(quote, r#"and <i>voilà</i>!"#);
/// ```
#[macro_export]
macro_rules! i {
    () => {
        $crate::html::HtmlI::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlI::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlI::new().content($crate::bake![$first $(, $rest)*])
    };
}
