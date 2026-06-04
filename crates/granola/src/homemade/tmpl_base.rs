use askama::Template;
use std::borrow::Cow;

use crate::{prelude::*, recipes::*, template::*};

/// The homemade recipe for the [`TmplBase`] template.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, template::*};
///
/// let tmpl: TmplBase<Homemade> = TmplBase::from_cookbook();
///
/// assert_eq!(
///     tmpl.bake(),
///     r#"<!doctype html>
/// <html>
///   <head>
///     <meta charset="utf-8" />
///     <meta name="viewport" content="width=device-width, initial-scale=1" />
///   </head>
///   <body></body>
/// </html>"#
/// );
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*, template::*};
///
/// let meta = HtmlMeta::from(NameRobots).content("noindex, nofollow");
/// let title = HtmlTitle::new().content("Home");
///
/// let css_rule = CssRule::new()
///     .selectors_list("body")
///     .declarations_block([("height", "100vh"), ("margin", "0")]);
/// let style = HtmlStyle::new().content(css_rule);
///
/// let body = HtmlBody::new().content(bake_newline!("Hello, world!"));
///
/// let tmpl = TmplBase::<Homemade>::from_cookbook()
///     .content(body)
///     .lang("en")
///     .push_meta(meta)
///     .push_title(title)
///     .push_style(style);
///
/// assert_eq!(
///     tmpl.bake(),
///     r#"<!doctype html>
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
/// </html>"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Homemade;

impl HtmlRecipe for Homemade {
    type Content = HomemadeRootContent;

    fn content_recipe(content: &mut Self::Content) {
        content.head = HtmlHead::from(Homemade);
        content.body.get_or_insert_with(HtmlBody::from_cookbook);
    }
}

/// One [`HtmlHead<Homemade>`], followed by an optional [`HtmlBody`].
///
/// The content of [`HtmlRoot<Homemade>`].
///
/// ```askama
/// {{ head }}
/// {% if let Some(b) = body -%}
/// {{ b }}
/// {%- endif -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HomemadeRootContent {
    pub head: HtmlHead<Homemade>,
    pub body: Option<HtmlBody>,
}

impl From<HomemadeRootContent> for HtmlRootContent {
    fn from(html_root_homemade_content: HomemadeRootContent) -> Self {
        Self {
            head: Some(html_root_homemade_content.head.bake_recipe()),
            body: html_root_homemade_content.body,
        }
    }
}

impl From<HtmlBody> for HomemadeRootContent {
    fn from(body: HtmlBody) -> Self {
        Self {
            head: HtmlHead::from(Homemade),
            body: Some(body),
        }
    }
}

impl HeadRecipe for Homemade {
    type Content = HomemadeHeadContent;

    fn content_recipe(content: &mut Self::Content) {
        content.meta.push(HtmlMeta::from(CharsetUtf8).bake_recipe());
        content.meta.push(
            HtmlMeta::from(NameViewport)
                .content("width=device-width, initial-scale=1")
                .bake_recipe(),
        );
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
pub struct HomemadeHeadContent {
    pub meta: Vec<HtmlMeta>,
    pub title: Option<HtmlTitle>,
    pub link: Vec<HtmlLink>,
    pub style: Vec<HtmlStyle>,
}

impl TmplBase<Homemade> {
    pub fn lang(mut self, lang: impl Into<Cow<'static, str>>) -> Self {
        self.html_root = self.html_root.lang(lang);
        self
    }

    pub fn push_meta<R: MetaRecipe>(mut self, meta: HtmlMeta<R>) -> Self {
        self.html_root
            .content
            .head
            .content
            .meta
            .push(meta.bake_recipe());
        self
    }

    pub fn push_title<R: TitleRecipe>(mut self, title: HtmlTitle<R>) -> Self {
        self.html_root.content.head.content.title = Some(title.bake_recipe());
        self
    }

    pub fn push_link<R: LinkRecipe>(mut self, link: HtmlLink<R>) -> Self {
        self.html_root
            .content
            .head
            .content
            .link
            .push(link.bake_recipe());
        self
    }

    pub fn push_style<R: StyleRecipe>(mut self, style: HtmlStyle<R>) -> Self {
        self.html_root
            .content
            .head
            .content
            .style
            .push(style.bake_recipe());
        self
    }
}
