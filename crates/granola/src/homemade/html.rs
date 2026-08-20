use askama::Template;

use crate::{prelude::*, recipes::*};

/// The homemade content for a [`HtmlDocument`].
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let meta = HtmlMeta::from(NameRobots).content("noindex, nofollow");
/// let title = HtmlTitle::new().content("Home");
///
/// let css_rule = CssRule::new()
///     .push_selector("body")
///     .push_property(CssDeclaration::from(Height).value("100vh"))
///     .push_property(CssDeclaration::from(Margin).value("0"));
/// let style = HtmlStyle::new().content(css_rule);
///
/// let body = HtmlBody::new().content("Hello, world!");
///
/// let content = HomemadeRootContent::new()
///     .push_meta(meta)
///     .push_title(title)
///     .push_style(style)
///     .body(body);
///
/// let page = HtmlDocument::new().content(HtmlRoot::new().lang("en").content(content));
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
pub struct HomemadeRootContent {
    pub head: HomemadeHeadContent,
    pub body: Option<HtmlBody>,
}

impl HomemadeRootContent {
    pub fn new() -> Self {
        Self {
            head: HomemadeHeadContent::new(),
            body: None,
        }
    }

    pub fn body<R: BodyRecipe>(mut self, body: HtmlBody<R>) -> Self {
        self.body = Some(body.bake_recipe());
        self
    }

    pub fn push_meta<R: MetaRecipe>(mut self, meta: HtmlMeta<R>) -> Self {
        self.head = self.head.push_meta(meta);
        self
    }

    pub fn push_title<R: TitleRecipe>(mut self, title: HtmlTitle<R>) -> Self {
        self.head = self.head.push_title(title);
        self
    }

    pub fn push_link<R: LinkRecipe>(mut self, link: HtmlLink<R>) -> Self {
        self.head = self.head.push_link(link);
        self
    }

    pub fn push_style<R: StyleRecipe>(mut self, style: HtmlStyle<R>) -> Self {
        self.head = self.head.push_style(style);
        self
    }
}

impl From<HomemadeRootContent> for HtmlRootContent {
    fn from(html_root_homemade_content: HomemadeRootContent) -> Self {
        Self {
            head: Some(HtmlHead::new().content(html_root_homemade_content.head)),
            body: html_root_homemade_content.body,
        }
    }
}

impl From<HomemadeRootContent> for Bake {
    fn from(content: HomemadeRootContent) -> Self {
        HtmlRootContent::from(content).into()
    }
}

impl<R: BodyRecipe> From<HtmlBody<R>> for HomemadeRootContent {
    fn from(body: HtmlBody<R>) -> Self {
        Self {
            body: Some(body.bake_recipe()),
            ..Self::new()
        }
    }
}

/// The [`HtmlHead`] content used by [`HomemadeRootContent`].
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
    pub fn new() -> Self {
        Self {
            meta: vec![
                HtmlMeta::from(CharsetUtf8).bake_recipe(),
                HtmlMeta::from(NameViewport)
                    .content("width=device-width, initial-scale=1")
                    .bake_recipe(),
            ],
            ..Default::default()
        }
    }

    pub fn push_meta<R: MetaRecipe>(mut self, meta: HtmlMeta<R>) -> Self {
        self.meta.push(meta.bake_recipe());
        self
    }

    pub fn push_title<R: TitleRecipe>(mut self, title: HtmlTitle<R>) -> Self {
        self.title = Some(title.bake_recipe());
        self
    }

    pub fn push_link<R: LinkRecipe>(mut self, link: HtmlLink<R>) -> Self {
        self.link.push(link.bake_recipe());
        self
    }

    pub fn push_style<R: StyleRecipe>(mut self, style: HtmlStyle<R>) -> Self {
        self.style.push(style.bake_recipe());
        self
    }
}
