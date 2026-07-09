use askama::Template;

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
///     .push_selector("body")
///     .push_property(CssDeclaration::from(Height).content("100vh"))
///     .push_property(CssDeclaration::from(Margin).content("0"));
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

impl HtmlDocument<Homemade> {
    pub fn lang(mut self, lang: impl Into<Bake>) -> Self {
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

impl HtmlDocumentRecipe for Homemade {
    type Content = HtmlRoot<Homemade>;

    fn bake_content(content: Self::Content) -> HtmlRoot {
        content.bake_recipe()
    }

    fn content_recipe() -> Self::Content {
        HtmlRoot::from(Homemade)
    }
}

impl HtmlRecipe for Homemade {
    recipe_boilerplate!(HtmlRecipe, HomemadeRootContent);

    fn content_recipe() -> Self::Content {
        Self::Content {
            head: HtmlHead::from(Homemade),
            ..Default::default()
        }
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
#[derive(Default, Debug, Clone, PartialEq, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HomemadeRootContent {
    pub head: HtmlHead<Homemade>,
    pub body: Option<HtmlBody>,
}

impl HomemadeRootContent {
    pub fn push_meta<R: MetaRecipe>(&mut self, meta: HtmlMeta<R>) -> &mut Self {
        self.head.content.push_meta(meta);
        self
    }

    pub fn push_title<R: TitleRecipe>(&mut self, title: HtmlTitle<R>) -> &mut Self {
        self.head.content.push_title(title);
        self
    }

    pub fn push_link<R: LinkRecipe>(&mut self, link: HtmlLink<R>) -> &mut Self {
        self.head.content.push_link(link);
        self
    }

    pub fn push_style<R: StyleRecipe>(&mut self, style: HtmlStyle<R>) -> &mut Self {
        self.head.content.push_style(style);
        self
    }

    pub fn body<R: BodyRecipe>(&mut self, body: HtmlBody<R>) -> &mut Self {
        self.body = Some(body.bake_recipe());
        self
    }
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

    fn content_recipe() -> Self::Content {
        let mut content = Self::Content::default();
        content
            .push_meta(HtmlMeta::from(CharsetUtf8))
            .push_meta(HtmlMeta::from(NameViewport).content("width=device-width, initial-scale=1"));
        content
    }
}

impl HtmlRoot<Homemade> {
    pub fn push_meta<R: MetaRecipe>(&mut self, meta: HtmlMeta<R>) -> &mut Self {
        self.content.push_meta(meta);
        self
    }

    pub fn push_title<R: TitleRecipe>(&mut self, title: HtmlTitle<R>) -> &mut Self {
        self.content.push_title(title);
        self
    }

    pub fn push_link<R: LinkRecipe>(&mut self, link: HtmlLink<R>) -> &mut Self {
        self.content.push_link(link);
        self
    }

    pub fn push_style<R: StyleRecipe>(&mut self, style: HtmlStyle<R>) -> &mut Self {
        self.content.push_style(style);
        self
    }

    pub fn body<R: BodyRecipe>(&mut self, body: HtmlBody<R>) -> &mut Self {
        self.content.body(body);
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
#[derive(Default, Debug, Clone, PartialEq, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HomemadeHeadContent {
    pub meta: Vec<HtmlMeta>,
    pub title: Option<HtmlTitle>,
    pub link: Vec<HtmlLink>,
    pub style: Vec<HtmlStyle>,
}

impl HomemadeHeadContent {
    pub fn push_meta<R: MetaRecipe>(&mut self, meta: HtmlMeta<R>) -> &mut Self {
        self.meta.push(meta.bake_recipe());
        self
    }

    pub fn push_title<R: TitleRecipe>(&mut self, title: HtmlTitle<R>) -> &mut Self {
        self.title = Some(title.bake_recipe());
        self
    }

    pub fn push_link<R: LinkRecipe>(&mut self, link: HtmlLink<R>) -> &mut Self {
        self.link.push(link.bake_recipe());
        self
    }

    pub fn push_style<R: StyleRecipe>(&mut self, style: HtmlStyle<R>) -> &mut Self {
        self.style.push(style.bake_recipe());
        self
    }
}
