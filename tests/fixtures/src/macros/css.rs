use granola::{homemade::*, macros::*, prelude::*, recipes::*};

pub fn style() -> CssStylesheet {
    CssStylesheet::from(OatsAndEndsMacros).bake_recipe()
}

#[derive(Default, Debug, Clone, PartialEq)]
struct OatsAndEndsMacros;

impl StylesheetRecipe for OatsAndEndsMacros {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssStylesheet::from(AndyBell),
            palette(),
            palette_dark(),
            CssStylesheet::from(Btn),
            CssStylesheet::from(Tooltip),
            typography(),
            layout(),
            skip_link(),
            site_header(),
            hero(),
            menu(),
            hours(),
            visit(),
            newsletter(),
            site_footer(),
        ]
    }
}

fn palette() -> CssRule {
    rule!(
        ":root",
        declarations_block![
            (ColorBackground, "#fbf4e8"),
            (ColorSurface, "#ffffff"),
            (ColorBorder, "#e4d3b6"),
            (ColorText, "#2c1c12"),
            (ColorPrimary, "#96491f"),
            (ColorPrimaryText, "#fffaf5"),
            (ColorError, "#9a2f22"),
            (ColorSuccess, "#2f6b46"),
        ]
    )
}

fn palette_dark() -> Bake {
    CssAtRule::new()
        .identifier("media")
        .rule("(prefers-color-scheme: dark)")
        .block(rule!(
            ":root",
            declarations_block![
                (ColorBackground, "#1c140d"),
                (ColorSurface, "#271c13"),
                (ColorBorder, "#4a3823"),
                (ColorText, "#f3e7d6"),
                (ColorPrimary, "#e2905c"),
                (ColorPrimaryText, "#221207"),
                (ColorError, "#e2897d"),
                (ColorSuccess, "#83c99b"),
            ]
        ))
        .into()
}

fn muted_text() -> CssFnColorMix {
    CssFnColorMix::new()
        .interpolation(ColorSpace::Oklab)
        .color_pct(CssFnVar::from(ColorText), "70%")
        .color("transparent")
}

fn selection_tint() -> CssFnColorMix {
    CssFnColorMix::new()
        .interpolation(ColorSpace::Oklab)
        .color_pct(CssFnVar::from(ColorPrimary), "30%")
        .color("transparent")
}

fn typography() -> Bake {
    bake_ws![
        rule!(
            "body",
            declarations_block![
                (FontFamily, "system-ui, sans-serif"),
                (Color, CssFnVar::from(ColorText)),
                (BackgroundColor, CssFnVar::from(ColorBackground)),
            ]
        ),
        CssRule::from((
            Headings,
            declarations_block![
                (FontFamily, "Georgia, 'Iowan Old Style', ui-serif, serif"),
                (FontWeight, "600"),
            ]
        )),
        rule!(
            "h1",
            declaration!(FontSize, "clamp(2rem, 1.5rem + 2vw, 3.25rem)")
        ),
        rule!("h2", declaration!(FontSize, "1.75rem")),
        rule!("h3", declaration!(FontSize, "1.25rem")),
        rule!(
            ".lede",
            declarations_block![(MaxWidth, "38rem"), (FontSize, "1.125rem")]
        ),
        rule!(
            ".note",
            declarations_block![
                (MaxWidth, "38rem"),
                (Color, muted_text()),
                (FontSize, "0.9rem"),
            ]
        ),
        rule!(
            "main a:not(.btn), .footer-inner a",
            declarations_block![
                (Color, CssFnVar::from(ColorPrimary)),
                ("text-underline-offset", "0.15em"),
            ]
        ),
        rule!(
            "::selection",
            declaration!(BackgroundColor, selection_tint())
        ),
    ]
}

fn layout() -> Bake {
    bake_ws![
        rule!(
            ".wrap",
            declarations_block![
                (MaxWidth, "68rem"),
                ("margin-inline", "auto"),
                ("padding-inline", "clamp(1rem, 4vw, 2.5rem)"),
            ]
        ),
        rule!(".section", declaration!(PaddingBlock, "3rem")),
        Bake::from(
            CssAtRule::new()
                .identifier("media")
                .rule("(prefers-reduced-motion: no-preference)")
                .block(rule!("html", declaration!("scroll-behavior", "smooth"))),
        ),
    ]
}

fn skip_link() -> Bake {
    bake_ws![
        rule!(
            ".skip-link",
            declarations_block![
                (Position, "absolute"),
                (Top, "-3rem"),
                ("left", "1rem"),
                (Padding, "0.5em 1em"),
                (BorderRadius, "0.35em"),
                (BackgroundColor, CssFnVar::from(ColorText)),
                (Color, CssFnVar::from(ColorBackground)),
                (TextDecoration, "none"),
                (ZIndex, "10"),
                (Transition, "top 150ms ease"),
            ]
        ),
        rule!(".skip-link:focus-visible", declaration!(Top, "1rem")),
    ]
}

fn site_header() -> Bake {
    bake_ws![
        rule!(
            ".site-header",
            declarations_block![
                (BorderBottom, "1px solid"),
                (BorderColor, CssFnVar::from(ColorBorder)),
                (Position, "sticky"),
                (Top, "0"),
                (ZIndex, "5"),
                (BackgroundColor, CssFnVar::from(ColorBackground)),
            ]
        ),
        rule!(
            ".site-nav",
            declarations_block![
                (Display, "flex"),
                (AlignItems, "center"),
                (JustifyContent, "space-between"),
                (Gap, "1rem"),
                (PaddingBlock, "1rem"),
            ]
        ),
        rule!(
            ".brand",
            declarations_block![
                (FontFamily, "Georgia, 'Iowan Old Style', ui-serif, serif"),
                (FontWeight, "700"),
                (FontSize, "1.25rem"),
                (Color, CssFnVar::from(ColorText)),
                (TextDecoration, "none"),
            ]
        ),
        rule!(
            ".nav-links",
            declarations_block![
                (Display, "flex"),
                (AlignItems, "center"),
                (Gap, "1.5rem"),
                (FlexWrap, "wrap"),
            ]
        ),
        rule!(
            ".nav-links a:not(.btn)",
            declarations_block![
                (Color, CssFnVar::from(ColorText)),
                (FontWeight, "500"),
                (TextDecoration, "none"),
            ]
        ),
        rule!(
            ".nav-links a:not(.btn):hover",
            declaration!(TextDecoration, "underline")
        ),
    ]
}

fn hero() -> Bake {
    bake_ws![
        rule!(
            ".hero",
            declarations_block![(PaddingBlock, "4rem 3rem"), (TextAlign, "center")]
        ),
        rule!(".hero .lede", declaration!(MaxWidth, "none")),
        rule!(
            ".hero-actions",
            declarations_block![
                (Display, "flex"),
                (JustifyContent, "center"),
                (Gap, "1rem"),
                (FlexWrap, "wrap"),
                (MarginTop, "2rem"),
            ]
        ),
    ]
}

fn menu() -> Bake {
    bake_ws![
        rule!(
            ".menu-groups",
            declarations_block![(Display, "grid"), (Gap, "3rem"), (MarginTop, "2rem")]
        ),
        Bake::from(
            CssAtRule::new()
                .identifier("media")
                .rule("(min-width: 720px)")
                .block(rule!(
                    ".menu-groups",
                    declaration!(GridTemplateColumns, "1fr 1fr")
                )),
        ),
        rule!(
            ".menu-group h3",
            declarations_block![
                (BorderBottom, "1px solid"),
                (BorderColor, CssFnVar::from(ColorBorder)),
                (Padding, "0 0 0.5rem"),
                (MarginBottom, "1rem"),
            ]
        ),
        rule!(
            ".menu-list",
            declarations_block![
                (Display, "flex"),
                (FlexDirection, "column"),
                (Gap, "1.25rem"),
                (Margin, "0"),
                (Padding, "0"),
            ]
        ),
        rule!(
            ".menu-item",
            declarations_block![
                (Display, "flex"),
                (FlexWrap, "wrap"),
                (JustifyContent, "space-between"),
                (AlignItems, "baseline"),
                (Gap, "0.25rem 1rem"),
            ]
        ),
        rule!(
            ".menu-item-name",
            declarations_block![
                (Display, "inline-flex"),
                (AlignItems, "center"),
                (Gap, "0.4rem"),
                (FontWeight, "600"),
            ]
        ),
        rule!(
            ".menu-item-desc",
            declarations_block![
                ("flex-basis", "100%"),
                (Color, muted_text()),
                (FontSize, "0.9rem"),
            ]
        ),
    ]
}

fn hours() -> Bake {
    bake_ws![
        rule!(
            "table",
            declarations_block![
                (Width, "100%"),
                (BorderCollapse, "collapse"),
                (MarginTop, "1.5rem")
            ]
        ),
        rule!(
            "caption",
            declarations_block![
                (TextAlign, "left"),
                (FontWeight, "600"),
                (MarginBottom, "0.75rem")
            ]
        ),
        rule!(
            bake_comma!["th", "td"],
            declarations_block![
                (TextAlign, "left"),
                (Padding, "0.6rem 1rem 0.6rem 0"),
                (BorderBottom, "1px solid"),
                (BorderColor, CssFnVar::from(ColorBorder)),
                ("font-variant-numeric", "tabular-nums"),
            ]
        ),
    ]
}

fn visit() -> Bake {
    bake_ws![
        rule!(
            "#visit address",
            declarations_block![
                (FontStyle, "normal"),
                (LineHeight, "1.7"),
                (MarginTop, "1rem")
            ]
        ),
        rule!(
            ".visit-actions",
            declarations_block![
                (Display, "flex"),
                (Gap, "1rem"),
                (FlexWrap, "wrap"),
                (MarginTop, "1.5rem")
            ]
        ),
    ]
}

fn newsletter() -> Bake {
    bake_ws![
        rule!(
            "#newsletter form",
            declarations_block![
                (Display, "flex"),
                (FlexWrap, "wrap"),
                (AlignItems, "flex-end"),
                (Gap, "1rem"),
                (MarginTop, "1.5rem"),
            ]
        ),
        rule!(
            ".field",
            declarations_block![
                (Display, "flex"),
                (FlexDirection, "column"),
                (Gap, "0.35rem")
            ]
        ),
        rule!(
            ".field label",
            declarations_block![(FontSize, "0.875rem"), (FontWeight, "500")]
        ),
        rule!(
            ".field input",
            declarations_block![
                (Padding, "0.6em 0.8em"),
                (Border, "1px solid"),
                (BorderColor, CssFnVar::from(ColorBorder)),
                (BorderRadius, "0.5em"),
                (BackgroundColor, CssFnVar::from(ColorSurface)),
                (Color, CssFnVar::from(ColorText)),
                (FontSize, "1rem"),
                (MinWidth, "16rem"),
            ]
        ),
        rule!(
            ".field input:focus-visible",
            declarations_block![
                (OutlineWidth, "2px"),
                (OutlineStyle, "solid"),
                (OutlineColor, CssFnVar::from(ColorPrimary)),
                (OutlineOffset, "2px"),
            ]
        ),
    ]
}

fn site_footer() -> Bake {
    bake_ws![
        rule!(
            ".site-footer",
            declarations_block![
                (BorderTop, "1px solid"),
                (BorderColor, CssFnVar::from(ColorBorder)),
                (PaddingBlock, "2rem"),
            ]
        ),
        rule!(
            ".footer-inner",
            declarations_block![
                (Display, "flex"),
                (JustifyContent, "space-between"),
                (AlignItems, "center"),
                (FlexWrap, "wrap"),
                (Gap, "0.75rem"),
            ]
        ),
        rule!(
            ".footer-inner address",
            declarations_block![(FontStyle, "normal"), (FontSize, "0.875rem")]
        ),
    ]
}
