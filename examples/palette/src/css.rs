use std::sync::LazyLock;

use granola::{macros::*, prelude::*, recipes::*};

/// The stylesheet url.
///
/// The URL embeds a hash of the content, so it changes if and only if the CSS
/// changes. Combined with a long-lived `immutable` policy, a returning visitor
/// reuses the cached file with no network round-trip, while any edit produces a
/// brand-new URL that misses the cache and is fetched fresh.
///
/// See: [Cache Busting](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Caching#cache_busting)
pub static STYLESHEET_URL: LazyLock<String> = LazyLock::new(|| {
    format!(
        "/static/style.{:016x}.css",
        seahash::hash(STATIC_STYLESHEET.as_bytes())
    )
});

/// The static stylesheet with the layout CSS content.
pub static STATIC_STYLESHEET: LazyLock<String> = LazyLock::new(|| stylesheet().bake());

fn stylesheet() -> CssStylesheet {
    stylesheet!(
        rule!(":root", ("--border", "1px")),
        rule!(
            "body",
            declarations_block![
                CssBackgroundColor::new().content("var(--base-100)"),
                CssColor::new().content("var(--base-content)"),
                CssFontFamily::<SansSerif>::from_cookbook().bake_recipe(),
                CssDisplay::<Flex>::from_cookbook(),
                CssJustifyContent::<Center>::from_cookbook(),
                CssAlignItems::<Center>::from_cookbook().bake_recipe(),
                CssMinHeight::new().content("100vh"),
                CssMargin::new().content("0"),
            ]
        ),
        rule!(
            ".palette",
            declarations_block![
                CssBackground::new().content("var(--base-200)"),
                CssPadding::new().content("2rem"),
                CssBorderRadius::new().content("8px"),
                CssBoxShadow::new()
                    .content("0 1px 2px color-mix(in oklab, var(--base-content) 10%, #0000)",),
                CssTextAlign::<Center>::from_cookbook(),
            ]
        ),
        rule!(
            ".swatches",
            declarations_block![
                CssDisplay::<Flex>::from_cookbook(),
                CssFlexDirection::<Column>::from_cookbook(),
                CssGap::new().content("1rem"),
                CssJustifyContent::<Center>::from_cookbook(),
                CssMargin::new().content("1.5rem 0"),
                CssFlexWrap::<Wrap>::from_cookbook(),
            ]
        ),
        rule!(
            ".swatch",
            declarations_block![
                CssDisplay::<Flex>::from_cookbook(),
                CssFlexDirection::<Column>::from_cookbook(),
                CssAlignItems::<Center>::from_cookbook().bake_recipe(),
                CssGap::new().content("0.25rem"),
            ]
        ),
        rule!(
            ".square",
            declarations_block![
                CssWidth::new().content("64px"),
                CssHeight::new().content("64px"),
                CssBorderRadius::new().content("8px"),
                CssBoxShadow::new().content(
                    r#"/* TODO: fix formatting */
  0 0 0 var(--border) color-mix(in oklab, var(--base-content) 10%, #0000),
  0 1px color-mix(in oklab, var(--base-content) 10%, #0000) inset,
  0 -1px oklch(100% 0 0 / 0.1) inset"#,
                ),
            ]
        ),
        rule!(
            simple_selector!(".swatch").descendant("p"),
            declarations_block![
                CssFontSize::new().content("0.75rem"),
                CssMargin::new().content("0"),
            ]
        ),
        rule!(
            simple_selector!(".swatch")
                .descendant("p")
                .descendant("code"),
            CssFontSize::new().content("0.7rem")
        ),
        rule!(
            simple_selector!(".palette").descendant("form"),
            declarations_block![
                CssDisplay::<Flex>::from_cookbook(),
                CssGap::new().content("0.5rem"),
                CssJustifyContent::<Center>::from_cookbook(),
                CssAlignItems::from(Center).bake_recipe(),
                CssMarginTop::new().content("1rem"),
            ]
        ),
    )
}

#[cfg(test)]
mod palette_tests {
    use super::stylesheet;

    #[test]
    fn stylesheet_test() {
        assert_eq!(
            stylesheet().bake(),
            r#":root {
  --border: 1px;
}

body {
  background-color: var(--base-100);
  color: var(--base-content);
  font-family: sans-serif;
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  margin: 0;
}

.palette {
  background: var(--base-200);
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 1px 2px color-mix(in oklab, var(--base-content) 10%, #0000);
  text-align: center;
}

.swatches {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  justify-content: center;
  margin: 1.5rem 0;
  flex-wrap: wrap;
}

.swatch {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
}

.square {
  width: 64px;
  height: 64px;
  border-radius: 8px;
  box-shadow: /* TODO: fix formatting */
    0 0 0 var(--border) color-mix(in oklab, var(--base-content) 10%, #0000),
    0 1px color-mix(in oklab, var(--base-content) 10%, #0000) inset,
    0 -1px oklch(100% 0 0 / 0.1) inset;
}

.swatch p {
  font-size: 0.75rem;
  margin: 0;
}

.swatch p code {
  font-size: 0.7rem;
}

.palette form {
  display: flex;
  gap: 0.5rem;
  justify-content: center;
  align-items: center;
  margin-top: 1rem;
}"#
        );
    }
}
