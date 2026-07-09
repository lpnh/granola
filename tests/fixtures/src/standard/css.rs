use granola::{homemade::*, prelude::*, recipes::*};

pub fn style() -> CssStylesheet {
    CssStylesheet::from(OatsAndEndsStandard).bake_recipe()
}

#[derive(Default, Debug, Clone, PartialEq)]
struct OatsAndEndsStandard;

impl StylesheetRecipe for OatsAndEndsStandard {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        let mut content = Bake::default();
        content.fold_in_ws(CssStylesheet::from(AndyBell));
        content.fold_in_ws(palette());
        content.fold_in_ws(palette_dark());
        content.fold_in_ws(CssStylesheet::from(Btn));
        content.fold_in_ws(CssStylesheet::from(Tooltip));
        content.fold_in_ws(typography());
        content.fold_in_ws(layout());
        content.fold_in_ws(skip_link());
        content.fold_in_ws(site_header());
        content.fold_in_ws(hero());
        content.fold_in_ws(menu());
        content.fold_in_ws(hours());
        content.fold_in_ws(visit());
        content.fold_in_ws(newsletter());
        content.fold_in_ws(site_footer());
        content
    }
}

fn palette() -> CssRule {
    CssRule::new()
        .selectors_list(":root")
        .push_property((ColorBackground, "#fbf4e8"))
        .push_property((ColorSurface, "#ffffff"))
        .push_property((ColorBorder, "#e4d3b6"))
        .push_property((ColorText, "#2c1c12"))
        .push_property((ColorPrimary, "#96491f"))
        .push_property((ColorPrimaryText, "#fffaf5"))
        .push_property((ColorError, "#9a2f22"))
        .push_property((ColorSuccess, "#2f6b46"))
}

fn palette_dark() -> Bake {
    CssAtRule::new()
        .identifier("media")
        .rule("(prefers-color-scheme: dark)")
        .block(
            CssRule::new()
                .selectors_list(":root")
                .push_property((ColorBackground, "#1c140d"))
                .push_property((ColorSurface, "#271c13"))
                .push_property((ColorBorder, "#4a3823"))
                .push_property((ColorText, "#f3e7d6"))
                .push_property((ColorPrimary, "#e2905c"))
                .push_property((ColorPrimaryText, "#221207"))
                .push_property((ColorError, "#e2897d"))
                .push_property((ColorSuccess, "#83c99b")),
        )
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
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("body")
            .push_property((FontFamily, "system-ui, sans-serif"))
            .push_property((Color, CssFnVar::from(ColorText)))
            .push_property((BackgroundColor, CssFnVar::from(ColorBackground))),
    );
    content.fold_in_ws(
        CssRule::from(Headings)
            .push_property((FontFamily, "Georgia, 'Iowan Old Style', ui-serif, serif"))
            .push_property((FontWeight, "600")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("h1")
            .push_property((FontSize, "clamp(2rem, 1.5rem + 2vw, 3.25rem)")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("h2")
            .push_property((FontSize, "1.75rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("h3")
            .push_property((FontSize, "1.25rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".lede")
            .push_property((MaxWidth, "38rem"))
            .push_property((FontSize, "1.125rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".note")
            .push_property((MaxWidth, "38rem"))
            .push_property((Color, muted_text()))
            .push_property((FontSize, "0.9rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("main a:not(.btn), .footer-inner a")
            .push_property((Color, CssFnVar::from(ColorPrimary)))
            .push_property(("text-underline-offset", "0.15em")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("::selection")
            .push_property((BackgroundColor, selection_tint())),
    );
    content
}

fn layout() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".wrap")
            .push_property((MaxWidth, "68rem"))
            .push_property(("margin-inline", "auto"))
            .push_property(("padding-inline", "clamp(1rem, 4vw, 2.5rem)")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".section")
            .push_property((PaddingBlock, "3rem")),
    );
    content.fold_in_ws(Bake::from(
        CssAtRule::new()
            .identifier("media")
            .rule("(prefers-reduced-motion: no-preference)")
            .block(
                CssRule::new()
                    .selectors_list("html")
                    .push_property(("scroll-behavior", "smooth")),
            ),
    ));
    content
}

fn skip_link() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".skip-link")
            .push_property((Position, "absolute"))
            .push_property((Top, "-3rem"))
            .push_property(("left", "1rem"))
            .push_property((Padding, "0.5em 1em"))
            .push_property((BorderRadius, "0.35em"))
            .push_property((BackgroundColor, CssFnVar::from(ColorText)))
            .push_property((Color, CssFnVar::from(ColorBackground)))
            .push_property((TextDecoration, "none"))
            .push_property((ZIndex, "10"))
            .push_property((Transition, "top 150ms ease")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".skip-link:focus-visible")
            .push_property((Top, "1rem")),
    );
    content
}

fn site_header() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".site-header")
            .push_property((BorderBottom, "1px solid"))
            .push_property((BorderColor, CssFnVar::from(ColorBorder)))
            .push_property((Position, "sticky"))
            .push_property((Top, "0"))
            .push_property((ZIndex, "5"))
            .push_property((BackgroundColor, CssFnVar::from(ColorBackground))),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".site-nav")
            .push_property((Display, "flex"))
            .push_property((AlignItems, "center"))
            .push_property((JustifyContent, "space-between"))
            .push_property((Gap, "1rem"))
            .push_property((PaddingBlock, "1rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".brand")
            .push_property((FontFamily, "Georgia, 'Iowan Old Style', ui-serif, serif"))
            .push_property((FontWeight, "700"))
            .push_property((FontSize, "1.25rem"))
            .push_property((Color, CssFnVar::from(ColorText)))
            .push_property((TextDecoration, "none")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".nav-links")
            .push_property((Display, "flex"))
            .push_property((AlignItems, "center"))
            .push_property((Gap, "1.5rem"))
            .push_property((FlexWrap, "wrap")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".nav-links a:not(.btn)")
            .push_property((Color, CssFnVar::from(ColorText)))
            .push_property((FontWeight, "500"))
            .push_property((TextDecoration, "none")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".nav-links a:not(.btn):hover")
            .push_property((TextDecoration, "underline")),
    );
    content
}

fn hero() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".hero")
            .push_property((PaddingBlock, "4rem 3rem"))
            .push_property((TextAlign, "center")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".hero .lede")
            .push_property((MaxWidth, "none")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".hero-actions")
            .push_property((Display, "flex"))
            .push_property((JustifyContent, "center"))
            .push_property((Gap, "1rem"))
            .push_property((FlexWrap, "wrap"))
            .push_property((MarginTop, "2rem")),
    );
    content
}

fn menu() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".menu-groups")
            .push_property((Display, "grid"))
            .push_property((Gap, "3rem"))
            .push_property((MarginTop, "2rem")),
    );
    content.fold_in_ws(Bake::from(
        CssAtRule::new()
            .identifier("media")
            .rule("(min-width: 720px)")
            .block(
                CssRule::new()
                    .selectors_list(".menu-groups")
                    .push_property((GridTemplateColumns, "1fr 1fr")),
            ),
    ));
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".menu-group h3")
            .push_property((BorderBottom, "1px solid"))
            .push_property((BorderColor, CssFnVar::from(ColorBorder)))
            .push_property((Padding, "0 0 0.5rem"))
            .push_property((MarginBottom, "1rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".menu-list")
            .push_property((Display, "flex"))
            .push_property((FlexDirection, "column"))
            .push_property((Gap, "1.25rem"))
            .push_property((Margin, "0"))
            .push_property((Padding, "0")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".menu-item")
            .push_property((Display, "flex"))
            .push_property((FlexWrap, "wrap"))
            .push_property((JustifyContent, "space-between"))
            .push_property((AlignItems, "baseline"))
            .push_property((Gap, "0.25rem 1rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".menu-item-name")
            .push_property((Display, "inline-flex"))
            .push_property((AlignItems, "center"))
            .push_property((Gap, "0.4rem"))
            .push_property((FontWeight, "600")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".menu-item-desc")
            .push_property(("flex-basis", "100%"))
            .push_property((Color, muted_text()))
            .push_property((FontSize, "0.9rem")),
    );
    content
}

fn hours() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("table")
            .push_property((Width, "100%"))
            .push_property((BorderCollapse, "collapse"))
            .push_property((MarginTop, "1.5rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("caption")
            .push_property((TextAlign, "left"))
            .push_property((FontWeight, "600"))
            .push_property((MarginBottom, "0.75rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .push_selectors_list("th")
            .push_selectors_list("td")
            .push_property((TextAlign, "left"))
            .push_property((Padding, "0.6rem 1rem 0.6rem 0"))
            .push_property((BorderBottom, "1px solid"))
            .push_property((BorderColor, CssFnVar::from(ColorBorder)))
            .push_property(("font-variant-numeric", "tabular-nums")),
    );
    content
}

fn visit() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("#visit address")
            .push_property((FontStyle, "normal"))
            .push_property((LineHeight, "1.7"))
            .push_property((MarginTop, "1rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".visit-actions")
            .push_property((Display, "flex"))
            .push_property((Gap, "1rem"))
            .push_property((FlexWrap, "wrap"))
            .push_property((MarginTop, "1.5rem")),
    );
    content
}

fn newsletter() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list("#newsletter form")
            .push_property((Display, "flex"))
            .push_property((FlexWrap, "wrap"))
            .push_property((AlignItems, "flex-end"))
            .push_property((Gap, "1rem"))
            .push_property((MarginTop, "1.5rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".field")
            .push_property((Display, "flex"))
            .push_property((FlexDirection, "column"))
            .push_property((Gap, "0.35rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".field label")
            .push_property((FontSize, "0.875rem"))
            .push_property((FontWeight, "500")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".field input")
            .push_property((Padding, "0.6em 0.8em"))
            .push_property((Border, "1px solid"))
            .push_property((BorderColor, CssFnVar::from(ColorBorder)))
            .push_property((BorderRadius, "0.5em"))
            .push_property((BackgroundColor, CssFnVar::from(ColorSurface)))
            .push_property((Color, CssFnVar::from(ColorText)))
            .push_property((FontSize, "1rem"))
            .push_property((MinWidth, "16rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".field input:focus-visible")
            .push_property((OutlineWidth, "2px"))
            .push_property((OutlineStyle, "solid"))
            .push_property((OutlineColor, CssFnVar::from(ColorPrimary)))
            .push_property((OutlineOffset, "2px")),
    );
    content
}

fn site_footer() -> Bake {
    let mut content = Bake::default();
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".site-footer")
            .push_property((BorderTop, "1px solid"))
            .push_property((BorderColor, CssFnVar::from(ColorBorder)))
            .push_property((PaddingBlock, "2rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".footer-inner")
            .push_property((Display, "flex"))
            .push_property((JustifyContent, "space-between"))
            .push_property((AlignItems, "center"))
            .push_property((FlexWrap, "wrap"))
            .push_property((Gap, "0.75rem")),
    );
    content.fold_in_ws(
        CssRule::new()
            .selectors_list(".footer-inner address")
            .push_property((FontStyle, "normal"))
            .push_property((FontSize, "0.875rem")),
    );
    content
}
