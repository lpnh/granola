use askama::Template;
use std::borrow::Cow;

use crate::{prelude::*, recipes::*, templates::*};

/// The recipe for the [`TmplBase`] template.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*, templates::*};
///
/// let tmpl: TmplBase<Homemade> = TmplBase::from_recipe();
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
/// use granola::{recipes::*, prelude::*, templates::*};
///
/// let body: HtmlBody = HtmlBody::new(bake_newline!("Hello, world!"));
///
/// let tmpl: TmplBase<Homemade> = TmplBase::new(body)
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
pub struct Homemade;

impl HtmlTag for Homemade {
    type Content = HtmlRootContent<Homemade, ()>;

    fn content_recipe(content: &mut Self::Content) {
        content
            .head
            .get_or_insert_with(HtmlHead::<Homemade>::from_recipe);
        content.body.get_or_insert_with(HtmlBody::from_recipe);
    }

    fn decoration_recipe<R: HtmlTag>(element: HtmlRoot<R>) -> HtmlRoot<R> {
        element
    }
}

impl From<HtmlBody> for HtmlRootContent<Homemade, ()> {
    fn from(body: HtmlBody) -> Self {
        Self {
            head: Some(HtmlHead::<Homemade>::from_recipe()),
            body: Some(body),
        }
    }
}

impl HeadTag for Homemade {
    type Content = BaseHeadContent;

    fn content_recipe(content: &mut Self::Content) {
        content.meta.push(HtmlMeta::<Charset>::empty().bake());
        content
            .meta
            .push(HtmlMeta::<Viewport>::new("width=device-width, initial-scale=1").bake());
    }

    fn decoration_recipe<R: HeadTag>(element: HtmlHead<R>) -> HtmlHead<R> {
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

impl TmplBase<Homemade> {
    pub fn lang(mut self, lang: impl Into<Cow<'static, str>>) -> Self {
        self.html_root = self.html_root.lang(lang);
        self
    }

    pub fn meta<M: MetaTag>(mut self, meta: HtmlMeta<M>) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Homemade>::from_recipe)
            .content
            .meta
            .push(meta.bake());
        self
    }

    pub fn title(mut self, title: impl Into<Cow<'static, str>>) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Homemade>::from_recipe)
            .content
            .title = Some(HtmlTitle::new(title));
        self
    }

    pub fn style(mut self, style: HtmlStyle) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Homemade>::from_recipe)
            .content
            .style
            .push(style.bake());
        self
    }
}
