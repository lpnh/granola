use std::sync::LazyLock;
use strum::IntoEnumIterator;

use granola::{homemade::Garnish, macros::*, prelude::*, recipes::*};

/// A stylesheet paired with its content-hashed URL.
///
/// The URL embeds a hash of the content, so it changes if and only if the CSS
/// changes. Combined with a long-lived `immutable` policy, a returning visitor
/// reuses the cached file with no network round-trip, while any edit produces a
/// brand-new URL that misses the cache and is fetched fresh.
///
/// See: [Cache Busting](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Caching#cache_busting)
struct BakedStylesheet {
    body: String,
    url: String,
}

impl BakedStylesheet {
    fn new<R: StylesheetRecipe>(name: &str, baked_stylesheet: CssStylesheet<R>) -> Self {
        let body = baked_stylesheet.bake();

        let url = format!("/static/{name}.{:016x}.css", seahash::hash(body.as_bytes()));

        Self { body, url }
    }
}

#[derive(strum::EnumIter, Clone, Copy)]
pub enum Stylesheet {
    Cuisine,
    // resets
    AndyBell,
    JoshWComeau,
    ModernNormalize,
    Preflight,
}

impl Stylesheet {
    pub fn preload() {
        Self::iter().for_each(|s| {
            s.baked();
        });
    }

    pub fn body(self) -> &'static str {
        &self.baked().body
    }

    pub fn url(self) -> &'static str {
        &self.baked().url
    }

    pub fn link(self) -> HtmlLink<RelStylesheet> {
        HtmlLink::from(RelStylesheet).href(self.url())
    }

    fn baked(self) -> &'static BakedStylesheet {
        match self {
            Self::Cuisine => &CUISINE,
            Self::AndyBell => &ANDY_BELL,
            Self::JoshWComeau => &JOSH_W_COMEAU,
            Self::ModernNormalize => &MODERN_NORMALIZE,
            Self::Preflight => &PREFLIGHT,
        }
    }
}

static CUISINE: LazyLock<BakedStylesheet> =
    LazyLock::new(|| BakedStylesheet::new("cuisine", cuisine_stylesheet()));
static ANDY_BELL: LazyLock<BakedStylesheet> =
    LazyLock::new(|| BakedStylesheet::new("andy_bell", CssStylesheet::from(AndyBell)));
static JOSH_W_COMEAU: LazyLock<BakedStylesheet> =
    LazyLock::new(|| BakedStylesheet::new("josh_w_comeau", CssStylesheet::from(JoshWComeau)));
static MODERN_NORMALIZE: LazyLock<BakedStylesheet> = LazyLock::new(|| {
    BakedStylesheet::new("modern_normalize", CssStylesheet::from(ModernNormalize))
});
static PREFLIGHT: LazyLock<BakedStylesheet> =
    LazyLock::new(|| BakedStylesheet::new("preflight", CssStylesheet::from(Preflight)));

fn cuisine_stylesheet() -> CssStylesheet<Garnish> {
    stylesheet!(
        @cookbook Garnish;
        @push
        rule!(
            "body",
            declarations_block![
                CssBackgroundColor::new().content("var(--color-background)"),
                CssColor::new().content("var(--color-text)"),
                CssDisplay::from(Flex),
                CssFlexDirection::from(Column),
                CssAlignItems::from(Center),
                CssGap::new().content("2rem"),
            ]
        ),
        rule!(
            "main",
            declarations_block![
                CssBackground::new().content("var(--color-surface)"),
                CssBorder::new().content("1px solid var(--color-border)"),
                CssBorderRadius::new().content("1em"),
                CssPadding::new().content("2rem"),
                CssTextAlign::from(Center),
            ]
        ),
        rule!(
            ".swatches",
            declarations_block![
                CssDisplay::from(Flex),
                CssGap::new().content("1rem"),
                CssJustifyContent::from(Center),
                CssFlexWrap::from(Wrap),
                CssPadding::new().content("2rem"),
            ]
        ),
        rule!(
            ".swatch",
            declarations_block![
                CssDisplay::from(Flex),
                CssFlexDirection::from(Column),
                CssAlignItems::from(Center),
                CssGap::new().content("0.25rem"),
            ]
        ),
        rule!(
            ".square",
            declarations_block![
                CssBorderRadius::new().content(".25em"),
                CssWidth::new().content("64px"),
                CssHeight::new().content("64px"),
                CssBoxShadow::new().content("0 0 0 1px color-mix(in oklab, var(--color-text) 10%, #0000), 0 1px color-mix(in oklab, var(--color-text) 10%, #0000) inset, 0 -1px oklch(100% 0 0 / 0.1) inset"),
            ]
        ),
        rule!(
            simple_selector!(".swatch").descendant("p"),
            CssFontSize::new().content("0.75rem"),
        ),
        rule!(
            ".picker",
            declarations_block![
                CssDisplay::new().content("grid"),
                CssGap::new().content("1rem"),
            ]
        ),
    )
}
