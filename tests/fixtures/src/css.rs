use std::sync::LazyLock;

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
    body: &'static str,
    url: String,
}

impl BakedStylesheet {
    fn new(name: &str, body: &'static str) -> Self {
        let url = format!("/static/{name}.{:016x}.css", seahash::hash(body.as_bytes()));

        Self { body, url }
    }
}

#[derive(Clone, Copy)]
pub enum Stylesheet {
    OatsAndEnds,
}

impl Stylesheet {
    pub fn body(self) -> &'static str {
        self.baked().body
    }

    pub fn url(self) -> &'static str {
        &self.baked().url
    }

    pub fn link(self) -> HtmlLink<RelStylesheet> {
        HtmlLink::from(RelStylesheet).href(self.url())
    }

    fn baked(self) -> &'static BakedStylesheet {
        match self {
            Self::OatsAndEnds => &OATS_AND_ENDS,
        }
    }
}

static OATS_AND_ENDS: LazyLock<BakedStylesheet> =
    LazyLock::new(|| BakedStylesheet::new("oats_and_ends", include_str!("../output.css")));
