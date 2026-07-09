use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<body>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/body)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let body = HtmlBody::new().id("document_body");
///
/// assert_eq!(body.bake(), r#"<body id="document_body"></body>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let body = HtmlBody::new().content("flow content");
///
/// assert_eq!(body.bake(), r#"<body>flow content</body>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <body
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</body>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = BodyRecipe, content = Bake)]
pub struct HtmlBody<R: BodyRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: BodyRecipe<Content = Bake>> HtmlBody<R> {
    pub fn fold_in(mut self, content: impl FastWritable) -> Self {
        self.content.fold_in(content);
        self
    }
}

/// Shorthand for `HtmlBody`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!().id("document_body");
///
/// assert_eq!(body.bake(), r#"<body id="document_body"></body>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!("flow content");
///
/// assert_eq!(body.bake(), r#"<body>flow content</body>"#);
/// ```
#[macro_export]
macro_rules! body {
    () => {
        $crate::html::HtmlBody::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlBody::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlBody::new().content($crate::bake![$first $(, $rest)*])
    };

}
