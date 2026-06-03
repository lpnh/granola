use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<main>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/main)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let main = HtmlMain::new().id("main");
///
/// assert_eq!(main.bake(), r#"<main id="main"></main>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let main = HtmlMain::new().content("hello, world!");
///
/// assert_eq!(main.bake(), r#"<main>hello, world!</main>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <main
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</main>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MainRecipe, content = Cow<'static, str>)]
pub struct HtmlMain<R: MainRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlMain`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let main = main!().id("main");
///
/// assert_eq!(main.bake(), r#"<main id="main"></main>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let main = main!("hello, world!");
///
/// assert_eq!(main.bake(), r#"<main>hello, world!</main>"#);
/// ```
#[macro_export]
macro_rules! main {
    () => {
        $crate::html::HtmlMain::new()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlMain::new().content($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlMain::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlMain::new().content($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlMain::new().content($crate::bake_inline![$($content),+])
    };
}
