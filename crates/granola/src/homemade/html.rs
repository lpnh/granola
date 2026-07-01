use askama::Template;
use std::borrow::Cow;

use crate::{prelude::*, recipes::*};

/// The homemade recipe for the [`HtmlDocument`].
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let page = HtmlDocument::from(Homemade);
///
/// assert_eq!(
///     page.bake_pretty(),
///     r#"<!DOCTYPE html>
/// <html>
///   <head>
///     <meta charset="utf-8" />
///     <meta name="viewport" content="width=device-width, initial-scale=1" />
///   </head>
///   <body></body>
/// </html>
/// "#
/// );
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let meta = HtmlMeta::from(NameRobots).content("noindex, nofollow");
/// let title = HtmlTitle::new().content("Home");
///
/// let css_rule = CssRule::new()
///     .selectors_list("body")
///     .declarations_block([("height", "100vh"), ("margin", "0")]);
/// let style = HtmlStyle::new().content(css_rule);
///
/// let body = HtmlBody::new().content("Hello, world!");
///
/// let page = HtmlDocument::from(Homemade)
///     .lang("en")
///     .push_meta(meta)
///     .push_title(title)
///     .push_style(style)
///     .body(body);
///
/// assert_eq!(
///     page.bake_pretty(),
///     r#"<!DOCTYPE html>
/// <html lang="en">
///   <head>
///     <meta charset="utf-8" />
///     <meta name="viewport" content="width=device-width, initial-scale=1" />
///     <meta name="robots" content="noindex, nofollow" />
///     <title>Home</title>
///     <style>
///     body {
///       height: 100vh;
///       margin: 0;
///     }
///     </style>
///   </head>
///   <body>Hello, world!</body>
/// </html>
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Homemade;

impl HtmlDocumentRecipe for Homemade {
    type Content = HtmlRoot<Homemade>;

    fn bake_content(content: Self::Content) -> HtmlRoot {
        content.bake_recipe()
    }

    fn content_recipe(content: &mut Self::Content) {
        *content = HtmlRoot::from(Homemade);
    }
}

impl HtmlRecipe for Homemade {
    recipe_boilerplate!(HtmlRecipe, HomemadeRootContent);

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
/// {%- if let Some(b) = body -%}
///     {{ b }}
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
            body: Some(body),
            ..Default::default()
        }
    }
}

impl HeadRecipe for Homemade {
    recipe_boilerplate!(HeadRecipe, HomemadeHeadContent);

    fn content_recipe(content: &mut Self::Content) {
        content.meta.push(HtmlMeta::from(CharsetUtf8).bake_recipe());
        content.meta.push(
            HtmlMeta::from(NameViewport)
                .content("width=device-width, initial-scale=1")
                .bake_recipe(),
        );
    }
}

impl HtmlRoot<Homemade> {
    pub fn push_meta<R: MetaRecipe>(&mut self, meta: HtmlMeta<R>) -> &mut Self {
        self.content.head.content.meta.push(meta.bake_recipe());
        self
    }

    pub fn push_title<R: TitleRecipe>(&mut self, title: HtmlTitle<R>) -> &mut Self {
        self.content.head.content.title = Some(title.bake_recipe());
        self
    }

    pub fn push_link<R: LinkRecipe>(&mut self, link: HtmlLink<R>) -> &mut Self {
        self.content.head.content.link.push(link.bake_recipe());
        self
    }

    pub fn push_style<R: StyleRecipe>(&mut self, style: HtmlStyle<R>) -> &mut Self {
        self.content.head.content.style.push(style.bake_recipe());
        self
    }

    pub fn body<R: BodyRecipe>(&mut self, body: HtmlBody<R>) -> &mut Self {
        self.content.body = Some(body.bake_recipe());
        self
    }
}

/// The [`HtmlHead`] content for the [`Homemade`] recipe.
///
/// # Askama template
///
/// ```askama
/// {%- for m in meta -%}
///     {{ m }}
/// {%- endfor -%}
/// {%- if let Some(t) = title -%}
///     {{ t }}
/// {%- endif -%}
/// {%- for l in link -%}
///     {{ l }}
/// {%- endfor -%}
/// {%- for s in style -%}
///     {{ s }}
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

impl HtmlDocument<Homemade> {
    pub fn lang(mut self, lang: impl Into<Cow<'static, str>>) -> Self {
        self.content = self.content.lang(lang);
        self
    }

    pub fn push_meta<R: MetaRecipe>(mut self, meta: HtmlMeta<R>) -> Self {
        self.content.push_meta(meta);
        self
    }

    pub fn push_title<R: TitleRecipe>(mut self, title: HtmlTitle<R>) -> Self {
        self.content.push_title(title);
        self
    }

    pub fn push_link<R: LinkRecipe>(mut self, link: HtmlLink<R>) -> Self {
        self.content.push_link(link);
        self
    }

    pub fn push_style<R: StyleRecipe>(mut self, style: HtmlStyle<R>) -> Self {
        self.content.push_style(style);
        self
    }

    pub fn body<R: BodyRecipe>(mut self, body: HtmlBody<R>) -> Self {
        self.content.body(body);
        self
    }
}
