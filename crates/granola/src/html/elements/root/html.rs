use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<html>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/html)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let html = HtmlRoot::new().id("html_document");
///
/// assert_eq!(html.bake(), r#"<html id="html_document"></html>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let body = HtmlBody::new().content("flow content");
///
/// let html = HtmlRoot::new().content(body).lang("en");
///
/// assert_eq!(
///     html.bake(),
///     r#"<html lang="en"><body>flow content</body></html>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let meta = HtmlMeta::new().charset();
/// let head = HtmlHead::new().content(meta);
///
/// let p = HtmlP::new().content("Hello, world!");
/// let body = HtmlBody::new().content(p);
///
/// let html = HtmlRoot::new().content((head, body));
///
/// assert_eq!(
///     html.bake(),
///     r#"<html><head><meta charset="utf-8" /></head><body><p>Hello, world!</p></body></html>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <html
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</html>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = HtmlRecipe, content = HtmlRootContent)]
pub struct HtmlRoot<R: HtmlRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// One HTML `<head>` element, followed by one `<body>` element.
///
/// The content of [`HtmlRoot`].
///
/// ```askama
/// {%- if let Some(h) = head -%}
///     {{ h }}
/// {%- endif -%}
/// {%- if let Some(b) = body -%}
///     {{ b }}
/// {%- endif -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlRootContent {
    pub head: Option<HtmlHead>,
    pub body: Option<HtmlBody>,
}

impl<H: HeadRecipe, B: BodyRecipe> From<(HtmlHead<H>, HtmlBody<B>)> for HtmlRoot {
    fn from(html_root_content: (HtmlHead<H>, HtmlBody<B>)) -> Self {
        Self::new().content(HtmlRootContent::from(html_root_content))
    }
}

impl<H: HeadRecipe> From<HtmlHead<H>> for HtmlRoot {
    fn from(head: HtmlHead<H>) -> Self {
        Self::new().content(HtmlRootContent::from(head))
    }
}

impl<B: BodyRecipe> From<HtmlBody<B>> for HtmlRoot {
    fn from(body: HtmlBody<B>) -> Self {
        Self::new().content(HtmlRootContent::from(body))
    }
}

impl<H: HeadRecipe, B: BodyRecipe> From<(HtmlHead<H>, HtmlBody<B>)> for HtmlRootContent {
    fn from((head, body): (HtmlHead<H>, HtmlBody<B>)) -> Self {
        Self {
            head: Some(head.bake_recipe()),
            body: Some(body.bake_recipe()),
        }
    }
}

impl<H: HeadRecipe> From<HtmlHead<H>> for HtmlRootContent {
    fn from(head: HtmlHead<H>) -> Self {
        Self {
            head: Some(head.bake_recipe()),
            body: None,
        }
    }
}

impl<B: BodyRecipe> From<HtmlBody<B>> for HtmlRootContent {
    fn from(body: HtmlBody<B>) -> Self {
        Self {
            head: None,
            body: Some(body.bake_recipe()),
        }
    }
}

/// Shorthand for `HtmlRoot`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let html = root!().id("html_document");
///
/// assert_eq!(html.bake(), r#"<html id="html_document"></html>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let body = body!("flow content");
///
/// let html = root!(body).lang("en");
///
/// assert_eq!(
///     html.bake(),
///     r#"<html lang="en"><body>flow content</body></html>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let meta = meta!().charset();
/// let head = head!(meta);
///
/// let p = HtmlP::new().content("Hello, world!");
/// let body = body!(p);
///
/// let html = root!(head, body);
///
/// assert_eq!(
///     html.bake(),
///     r#"<html><head><meta charset="utf-8" /></head><body><p>Hello, world!</p></body></html>"#
/// );
/// ```
#[macro_export]
macro_rules! root {
    () => {
        $crate::html::HtmlRoot::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlRoot::new().content($content)
    };
    ($head:expr, $body:expr $(,)?) => {
        $crate::html::HtmlRoot::new().content(($head, $body))
    };
}
