use std::sync::LazyLock;
use strum::IntoEnumIterator;

use granola::{macros::*, prelude::*, recipes::*};

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

fn cuisine_stylesheet() -> CssStylesheet {
    stylesheet!(
        rule!(":root", ("--border", "1px")),
        rule!(
            "body",
            declarations_block![
                CssBackgroundColor::new().content("var(--base-100)"),
                CssColor::new().content("var(--base-content)"),
                CssDisplay::from(Flex),
                CssFlexDirection::from(Column),
                CssAlignItems::from(Center),
                CssGap::new().content("2rem"),
            ]
        ),
        rule!(
            ".palette",
            declarations_block![
                CssBackground::new().content("var(--base-200)"),
                CssDisplay::from(Flex),
                CssFlexDirection::from(Column),
                CssAlignItems::from(Center),
                CssGap::new().content("1.5rem"),
                CssPadding::new().content("2rem"),
                CssBoxShadow::new()
                    .content("0 1px 2px color-mix(in oklab, var(--base-content) 10%, #0000)",),
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
                CssWidth::new().content("64px"),
                CssHeight::new().content("64px"),
                CssBoxShadow::new().content("0 0 0 var(--border) color-mix(in oklab, var(--base-content) 10%, #0000), 0 1px color-mix(in oklab, var(--base-content) 10%, #0000) inset, 0 -1px oklch(100% 0 0 / 0.1) inset"),
            ]
        ),
        rule!(
            simple_selector!(".swatch").descendant("p"),
            CssFontSize::new().content("0.75rem"),
        ),
        rule!(
            simple_selector!(".palette").descendant("form"),
            declarations_block![
                CssDisplay::from(Flex),
                CssGap::new().content("0.5rem"),
                CssJustifyContent::from(Center),
                CssAlignItems::from(Center),
            ]
        ),
    )
}

#[cfg(test)]
mod cuisine_tests {
    use super::cuisine_stylesheet;

    #[test]
    fn stylesheet_test() {
        assert_eq!(
            cuisine_stylesheet().bake(),
            r#":root { --border: 1px; } body { background-color: var(--base-100); color: var(--base-content); display: flex; flex-direction: column; align-items: center; gap: 2rem; } .palette { background: var(--base-200); display: flex; flex-direction: column; align-items: center; gap: 1.5rem; padding: 2rem; box-shadow: 0 1px 2px color-mix(in oklab, var(--base-content) 10%, #0000); text-align: center; } .swatches { display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap; } .swatch { display: flex; flex-direction: column; align-items: center; gap: 0.25rem; } .square { width: 64px; height: 64px; box-shadow: 0 0 0 var(--border) color-mix(in oklab, var(--base-content) 10%, #0000), 0 1px color-mix(in oklab, var(--base-content) 10%, #0000) inset, 0 -1px oklch(100% 0 0 / 0.1) inset; } .swatch p { font-size: 0.75rem; } .palette form { display: flex; gap: 0.5rem; justify-content: center; align-items: center; }"#
        );
    }
}
