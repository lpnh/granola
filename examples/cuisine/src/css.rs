use std::borrow::Cow;
use std::sync::LazyLock;
use strum::IntoEnumIterator;

use granola::{prelude::*, recipes::*};

/// A stylesheet paired with its content-hashed URL.
///
/// The URL embeds a hash of the content, so it changes if and only if the CSS
/// changes. Combined with a long-lived `immutable` policy, a returning visitor
/// reuses the cached file with no network round-trip, while any edit produces a
/// brand-new URL that misses the cache and is fetched fresh.
///
/// See: [Cache Busting](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Caching#cache_busting)
struct BakedStylesheet {
    body: Cow<'static, str>,
    url: String,
}

impl BakedStylesheet {
    fn new(name: &str, body: impl Into<Cow<'static, str>>) -> Self {
        let body = body.into();
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
    LazyLock::new(|| BakedStylesheet::new("cuisine", include_str!("../output.css")));
static ANDY_BELL: LazyLock<BakedStylesheet> =
    LazyLock::new(|| BakedStylesheet::new("andy_bell", CssStylesheet::from(AndyBell).bake()));
static JOSH_W_COMEAU: LazyLock<BakedStylesheet> = LazyLock::new(|| {
    BakedStylesheet::new("josh_w_comeau", CssStylesheet::from(JoshWComeau).bake())
});
static MODERN_NORMALIZE: LazyLock<BakedStylesheet> = LazyLock::new(|| {
    BakedStylesheet::new(
        "modern_normalize",
        CssStylesheet::from(ModernNormalize).bake(),
    )
});
static PREFLIGHT: LazyLock<BakedStylesheet> =
    LazyLock::new(|| BakedStylesheet::new("preflight", CssStylesheet::from(Preflight).bake()));
