use askama::Template;
use std::borrow::Cow;

use crate::{prelude::*, recipes::*, templates::*};

/// The homemade recipe for the [`TmplBase`] template.
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
///
/// let meta: HtmlMeta<Robots> = HtmlMeta::new("noindex, nofollow");
/// let title: HtmlTitle = HtmlTitle::new("Home");
///
/// let css_rule = CssRule::new("body", [("height", "100vh"), ("margin", "0")]);
/// let style: HtmlStyle = HtmlStyle::new(css_rule);
///
/// let body: HtmlBody = HtmlBody::new(bake_newline!("Hello, world!"));
///
/// let tmpl: TmplBase<Homemade> = TmplBase::new(body)
///     .lang("en")
///     .push_meta(meta)
///     .push_title(title)
///     .push_style(style);
///
/// assert_eq!(tmpl.bake(),
/// r#"<!doctype html>
/// <html lang="en">
///   <head>
///     <meta charset="utf-8" />
///     <meta name="viewport" content="width=device-width, initial-scale=1" />
///     <meta name="robots" content="noindex, nofollow" />
///     <title>Home</title>
///     <style>
///       body {
///         height: 100vh;
///         margin: 0;
///       }
///     </style>
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
        content.meta.push(HtmlMeta::<Charset>::from_recipe().bake());
        content
            .meta
            .push(HtmlMeta::<Viewport>::new("width=device-width, initial-scale=1").bake());
    }
}

/// The [`HtmlHead`] content for the [`Homemade`] recipe.
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
/// {%- for l in link %}
/// {{ l }}
/// {%- endfor -%}
/// {%- for s in style %}
/// {{ s }}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct BaseHeadContent {
    pub meta: Vec<String>,
    pub title: Option<String>,
    pub link: Vec<String>,
    pub style: Vec<String>,
}

impl TmplBase<Homemade> {
    pub fn lang(mut self, lang: impl Into<Cow<'static, str>>) -> Self {
        self.html_root = self.html_root.lang(lang);
        self
    }

    pub fn push_meta<R: MetaTag>(mut self, meta: HtmlMeta<R>) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Homemade>::from_recipe)
            .content
            .meta
            .push(meta.bake());
        self
    }

    pub fn push_title<R: TitleTag>(mut self, title: HtmlTitle<R>) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Homemade>::from_recipe)
            .content
            .title = Some(title.bake());
        self
    }

    pub fn push_link<R: LinkTag>(mut self, link: HtmlLink<R>) -> Self {
        self.html_root
            .content
            .head
            .get_or_insert_with(HtmlHead::<Homemade>::from_recipe)
            .content
            .link
            .push(link.bake());
        self
    }

    pub fn push_style<R: StyleTag>(mut self, style: HtmlStyle<R>) -> Self {
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
