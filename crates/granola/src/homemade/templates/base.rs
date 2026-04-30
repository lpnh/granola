use askama::Template;
use std::borrow::Cow;

use crate::{homemade::*, prelude::*, templates::*};

/// The base template recipe
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, templates::*};
///
/// let tmpl: TmplBase<Base> = TmplBase::empty();
///
/// assert_eq!(tmpl.bake(),
/// r#"<!doctype html>
/// <html>
///   <head>
///     <meta charset="utf-8" />
///     <meta name="viewport" content="width=device-width, initial-scale=1" />
///   </head>
///   <body></body>
/// </html>"#);
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*, templates::*};
///
/// let body: HtmlBody = HtmlBody::new(bake_newline!("Hello, world!"));
///
/// let tmpl: TmplBase<Base> = TmplBase::new(body)
///     .lang("en")
///     .meta(HtmlMeta::<Robots>::new("noindex, nofollow"))
///     .title("Home")
///     .style(HtmlStyle::new("body { margin: 0; }"));
///
/// assert_eq!(tmpl.bake(),
/// r#"<!doctype html>
/// <html lang="en">
///   <head>
///     <meta charset="utf-8" />
///     <meta name="viewport" content="width=device-width, initial-scale=1" />
///     <meta name="robots" content="noindex, nofollow" />
///     <title>Home</title>
///     <style>body { margin: 0; }</style>
///   </head>
///   <body>
///     Hello, world!
///   </body>
/// </html>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Base;

impl HtmlTag for Base {
    type Content = HtmlRootContent<Base>;

    fn recipe(mut element: HtmlRoot<Self>) -> HtmlRoot<Self> {
        if element.content.head.is_none() {
            element.content.head = Some(HtmlHead::<Base>::empty());
        }
        if element.content.body.is_none() {
            element.content.body = Some(HtmlBody::empty());
        }
        element
    }
}

impl From<HtmlBody> for HtmlRootContent<Base> {
    fn from(body: HtmlBody) -> Self {
        Self {
            head: None,
            body: Some(body),
        }
    }
}

impl HeadTag for Base {
    type Content = BaseHeadContent;

    fn recipe(mut element: HtmlHead<Self>) -> HtmlHead<Self> {
        element
            .content
            .meta
            .push(HtmlMeta::<Charset>::empty().bake());
        element
            .content
            .meta
            .push(HtmlMeta::<Viewport>::new("width=device-width, initial-scale=1").bake());
        element
    }
}

/// [`HtmlHead`] content for the [`Base`] recipe
///
/// # Askama template
///
/// ```askama
/// {%- for m in meta %}
/// {{ m }}
/// {%- endfor -%}
/// {%- if let Some(t) = title %}
/// {{ t }}
/// {%- endif -%}
/// {%- for s in style %}
/// {{ s }}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct BaseHeadContent {
    pub meta: Vec<String>,
    pub title: Option<HtmlTitle>,
    pub style: Vec<String>,
}

impl TmplBase<Base> {
    pub fn lang(mut self, lang: impl Into<Cow<'static, str>>) -> Self {
        self.html_root = self.html_root.lang(lang);
        self
    }

    pub fn meta<M: MetaTag>(mut self, meta: HtmlMeta<M>) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Base>::empty)
            .content
            .meta
            .push(meta.bake());
        self
    }

    pub fn title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Base>::empty)
            .content
            .title = Some(HtmlTitle::new(title));
        self
    }

    pub fn style(mut self, style: HtmlStyle) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Base>::empty)
            .content
            .style
            .push(style.bake());
        self
    }
}
