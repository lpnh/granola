use granola::{homemade::*, prelude::*, recipes::*};

pub fn style() -> CssStylesheet {
    CssStylesheet::from(OatsAndEndsStandard).bake_recipe()
}

#[derive(Default, Debug, Clone, PartialEq)]
struct OatsAndEndsStandard;

impl StylesheetRecipe for OatsAndEndsStandard {
    recipe_boilerplate!(StylesheetRecipe, CssStylesheet);

    fn content_recipe() -> Self::Content {
        CssStylesheet::new()
            .push(CssStylesheet::from(AndyBell))
            .push(palette())
            .push(palette_dark())
            .push(CssStylesheet::from(Btn))
            .push(CssStylesheet::from(Tooltip))
            .push(typography())
            .push(layout())
            .push(skip_link())
            .push(site_header())
            .push(hero())
            .push(menu())
            .push(hours())
            .push(visit())
            .push(newsletter())
            .push(site_footer())
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

fn palette_dark_root() -> CssRule {
    CssRule::new()
        .selectors_list(":root")
        .push_property((ColorBackground, "#1c140d"))
        .push_property((ColorSurface, "#271c13"))
        .push_property((ColorBorder, "#4a3823"))
        .push_property((ColorText, "#f3e7d6"))
        .push_property((ColorPrimary, "#e2905c"))
        .push_property((ColorPrimaryText, "#221207"))
        .push_property((ColorError, "#e2897d"))
        .push_property((ColorSuccess, "#83c99b"))
}

fn palette_dark() -> CssAtRule {
    CssAtRule::new()
        .identifier("media")
        .rule("(prefers-color-scheme: dark)")
        .block(palette_dark_root())
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

fn body() -> CssRule {
    CssRule::new()
        .selectors_list("body")
        .push_property((FontFamily, "system-ui, sans-serif"))
        .push_property((Color, CssFnVar::from(ColorText)))
        .push_property((BackgroundColor, CssFnVar::from(ColorBackground)))
}

fn headings() -> CssRule<Headings> {
    CssRule::from(Headings)
        .push_property((FontFamily, "Georgia, 'Iowan Old Style', ui-serif, serif"))
        .push_property((FontWeight, "600"))
}

fn h1() -> CssRule {
    CssRule::new().selectors_list("h1").push_property((
        FontSize,
        CssFnClamp::new()
            .min("2rem")
            .val("1.5rem + 2vw")
            .max("3.25rem"),
    ))
}

fn h2() -> CssRule {
    CssRule::new()
        .selectors_list("h2")
        .push_property((FontSize, "1.75rem"))
}

fn h3() -> CssRule {
    CssRule::new()
        .selectors_list("h3")
        .push_property((FontSize, "1.25rem"))
}

fn lede() -> CssRule {
    CssRule::new()
        .selectors_list(".lede")
        .push_property((MaxWidth, "38rem"))
        .push_property((FontSize, "1.125rem"))
}

fn note() -> CssRule {
    CssRule::new()
        .selectors_list(".note")
        .push_property((MaxWidth, "38rem"))
        .push_property((Color, muted_text()))
        .push_property((FontSize, "0.9rem"))
}

fn body_links() -> CssRule {
    CssRule::new()
        .selectors_list("main a:not(.btn), .footer-inner a")
        .push_property((Color, CssFnVar::from(ColorPrimary)))
        .push_property((TextUnderlineOffset, "0.15em"))
}

fn selection() -> CssRule {
    CssRule::new()
        .selectors_list("::selection")
        .push_property((BackgroundColor, selection_tint()))
}

fn typography() -> CssStylesheet {
    CssStylesheet::new()
        .push(body())
        .push(headings())
        .push(h1())
        .push(h2())
        .push(h3())
        .push(lede())
        .push(note())
        .push(body_links())
        .push(selection())
}

fn wrap() -> CssRule {
    CssRule::new()
        .selectors_list(".wrap")
        .push_property((MaxWidth, "68rem"))
        .push_property((MarginInline, "auto"))
        .push_property((
            PaddingInline,
            CssFnClamp::new().min("1rem").val("4vw").max("2.5rem"),
        ))
}

fn section_padding() -> CssRule {
    CssRule::new()
        .selectors_list(".section")
        .push_property((PaddingBlock, "3rem"))
}

fn scroll_behavior_smooth() -> CssRule {
    CssRule::new()
        .selectors_list("html")
        .push_property((ScrollBehavior, "smooth"))
}

fn reduced_motion() -> CssAtRule {
    CssAtRule::new()
        .identifier("media")
        .rule("(prefers-reduced-motion: no-preference)")
        .block(scroll_behavior_smooth())
}

fn layout() -> CssStylesheet {
    CssStylesheet::new()
        .push(wrap())
        .push(section_padding())
        .push(reduced_motion())
}

fn skip_link_rule() -> CssRule {
    CssRule::new()
        .selectors_list(".skip-link")
        .push_property((Position, "absolute"))
        .push_property((Top, "-3rem"))
        .push_property((Left, "1rem"))
        .push_property((Padding, "0.5em 1em"))
        .push_property((BorderRadius, "0.35em"))
        .push_property((BackgroundColor, CssFnVar::from(ColorText)))
        .push_property((Color, CssFnVar::from(ColorBackground)))
        .push_property((TextDecoration, "none"))
        .push_property((ZIndex, "10"))
        .push_property((Transition, "top 150ms ease"))
}

fn skip_link_focus() -> CssRule {
    CssRule::new()
        .selectors_list(".skip-link:focus-visible")
        .push_property((Top, "1rem"))
}

fn skip_link() -> CssStylesheet {
    CssStylesheet::new()
        .push(skip_link_rule())
        .push(skip_link_focus())
}

fn site_header_rule() -> CssRule {
    CssRule::new()
        .selectors_list(".site-header")
        .push_property((BorderBottom, "1px solid"))
        .push_property((BorderColor, CssFnVar::from(ColorBorder)))
        .push_property((Position, "sticky"))
        .push_property((Top, "0"))
        .push_property((ZIndex, "5"))
        .push_property((BackgroundColor, CssFnVar::from(ColorBackground)))
}

fn site_nav() -> CssRule {
    CssRule::new()
        .selectors_list(".site-nav")
        .push_property((Display, "flex"))
        .push_property((AlignItems, "center"))
        .push_property((JustifyContent, "space-between"))
        .push_property((Gap, "1rem"))
        .push_property((PaddingBlock, "1rem"))
}

fn brand() -> CssRule {
    CssRule::new()
        .selectors_list(".brand")
        .push_property((FontFamily, "Georgia, 'Iowan Old Style', ui-serif, serif"))
        .push_property((FontWeight, "700"))
        .push_property((FontSize, "1.25rem"))
        .push_property((Color, CssFnVar::from(ColorText)))
        .push_property((TextDecoration, "none"))
}

fn nav_links() -> CssRule {
    CssRule::new()
        .selectors_list(".nav-links")
        .push_property((Display, "flex"))
        .push_property((AlignItems, "center"))
        .push_property((Gap, "1.5rem"))
        .push_property((FlexWrap, "wrap"))
}

fn nav_links_anchor() -> CssRule {
    CssRule::new()
        .selectors_list(".nav-links a:not(.btn)")
        .push_property((Color, CssFnVar::from(ColorText)))
        .push_property((FontWeight, "500"))
        .push_property((TextDecoration, "none"))
}

fn nav_links_anchor_hover() -> CssRule {
    CssRule::new()
        .selectors_list(".nav-links a:not(.btn):hover")
        .push_property((TextDecoration, "underline"))
}

fn site_header() -> CssStylesheet {
    CssStylesheet::new()
        .push(site_header_rule())
        .push(site_nav())
        .push(brand())
        .push(nav_links())
        .push(nav_links_anchor())
        .push(nav_links_anchor_hover())
}

fn hero_rule() -> CssRule {
    CssRule::new()
        .selectors_list(".hero")
        .push_property((PaddingBlock, "4rem 3rem"))
        .push_property((TextAlign, "center"))
}

fn hero_lede() -> CssRule {
    CssRule::new()
        .selectors_list(".hero .lede")
        .push_property((MaxWidth, "none"))
}

fn hero_actions() -> CssRule {
    CssRule::new()
        .selectors_list(".hero-actions")
        .push_property((Display, "flex"))
        .push_property((JustifyContent, "center"))
        .push_property((Gap, "1rem"))
        .push_property((FlexWrap, "wrap"))
        .push_property((MarginTop, "2rem"))
}

fn hero() -> CssStylesheet {
    CssStylesheet::new()
        .push(hero_rule())
        .push(hero_lede())
        .push(hero_actions())
}

fn menu_groups() -> CssRule {
    CssRule::new()
        .selectors_list(".menu-groups")
        .push_property((Display, "grid"))
        .push_property((Gap, "3rem"))
        .push_property((MarginTop, "2rem"))
}

fn menu_groups_columns() -> CssRule {
    CssRule::new()
        .selectors_list(".menu-groups")
        .push_property((GridTemplateColumns, "1fr 1fr"))
}

fn menu_groups_wide() -> CssAtRule {
    CssAtRule::new()
        .identifier("media")
        .rule("(min-width: 720px)")
        .block(menu_groups_columns())
}

fn menu_group_heading() -> CssRule {
    CssRule::new()
        .selectors_list(".menu-group h3")
        .push_property((BorderBottom, "1px solid"))
        .push_property((BorderColor, CssFnVar::from(ColorBorder)))
        .push_property((Padding, "0 0 0.5rem"))
        .push_property((MarginBottom, "1rem"))
}

fn menu_list() -> CssRule {
    CssRule::new()
        .selectors_list(".menu-list")
        .push_property((Display, "flex"))
        .push_property((FlexDirection, "column"))
        .push_property((Gap, "1.25rem"))
        .push_property((Margin, "0"))
        .push_property((Padding, "0"))
}

fn menu_item() -> CssRule {
    CssRule::new()
        .selectors_list(".menu-item")
        .push_property((Display, "flex"))
        .push_property((FlexWrap, "wrap"))
        .push_property((JustifyContent, "space-between"))
        .push_property((AlignItems, "baseline"))
        .push_property((Gap, "0.25rem 1rem"))
}

fn menu_item_name() -> CssRule {
    CssRule::new()
        .selectors_list(".menu-item-name")
        .push_property((Display, "inline-flex"))
        .push_property((AlignItems, "center"))
        .push_property((Gap, "0.4rem"))
        .push_property((FontWeight, "600"))
}

fn menu_item_desc() -> CssRule {
    CssRule::new()
        .selectors_list(".menu-item-desc")
        .push_property((FlexBasis, "100%"))
        .push_property((Color, muted_text()))
        .push_property((FontSize, "0.9rem"))
}

fn menu() -> CssStylesheet {
    CssStylesheet::new()
        .push(menu_groups())
        .push(menu_groups_wide())
        .push(menu_group_heading())
        .push(menu_list())
        .push(menu_item())
        .push(menu_item_name())
        .push(menu_item_desc())
}

fn hours_table() -> CssRule {
    CssRule::new()
        .selectors_list("table")
        .push_property((Width, "100%"))
        .push_property((BorderCollapse, "collapse"))
        .push_property((MarginTop, "1.5rem"))
}

fn hours_caption() -> CssRule {
    CssRule::new()
        .selectors_list("caption")
        .push_property((TextAlign, "left"))
        .push_property((FontWeight, "600"))
        .push_property((MarginBottom, "0.75rem"))
}

fn hours_cells() -> CssRule {
    CssRule::new()
        .push_selectors_list("th")
        .push_selectors_list("td")
        .push_property((TextAlign, "left"))
        .push_property((Padding, "0.6rem 1rem 0.6rem 0"))
        .push_property((BorderBottom, "1px solid"))
        .push_property((BorderColor, CssFnVar::from(ColorBorder)))
        .push_property((FontVariantNumeric, "tabular-nums"))
}

fn hours() -> CssStylesheet {
    CssStylesheet::new()
        .push(hours_table())
        .push(hours_caption())
        .push(hours_cells())
}

fn visit_address() -> CssRule {
    CssRule::new()
        .selectors_list("#visit address")
        .push_property((FontStyle, "normal"))
        .push_property((LineHeight, "1.7"))
        .push_property((MarginTop, "1rem"))
}

fn visit_actions() -> CssRule {
    CssRule::new()
        .selectors_list(".visit-actions")
        .push_property((Display, "flex"))
        .push_property((Gap, "1rem"))
        .push_property((FlexWrap, "wrap"))
        .push_property((MarginTop, "1.5rem"))
}

fn visit() -> CssStylesheet {
    CssStylesheet::new()
        .push(visit_address())
        .push(visit_actions())
}

fn newsletter_form() -> CssRule {
    CssRule::new()
        .selectors_list("#newsletter form")
        .push_property((Display, "flex"))
        .push_property((FlexWrap, "wrap"))
        .push_property((AlignItems, "flex-end"))
        .push_property((Gap, "1rem"))
        .push_property((MarginTop, "1.5rem"))
}

fn field() -> CssRule {
    CssRule::new()
        .selectors_list(".field")
        .push_property((Display, "flex"))
        .push_property((FlexDirection, "column"))
        .push_property((Gap, "0.35rem"))
}

fn field_label() -> CssRule {
    CssRule::new()
        .selectors_list(".field label")
        .push_property((FontSize, "0.875rem"))
        .push_property((FontWeight, "500"))
}

fn field_input() -> CssRule {
    CssRule::new()
        .selectors_list(".field input")
        .push_property((Padding, "0.6em 0.8em"))
        .push_property((Border, "1px solid"))
        .push_property((BorderColor, CssFnVar::from(ColorBorder)))
        .push_property((BorderRadius, "0.5em"))
        .push_property((BackgroundColor, CssFnVar::from(ColorSurface)))
        .push_property((Color, CssFnVar::from(ColorText)))
        .push_property((FontSize, "1rem"))
        .push_property((MinWidth, "16rem"))
}

fn field_input_focus() -> CssRule {
    CssRule::new()
        .selectors_list(".field input:focus-visible")
        .push_property((OutlineWidth, "2px"))
        .push_property((OutlineStyle, "solid"))
        .push_property((OutlineColor, CssFnVar::from(ColorPrimary)))
        .push_property((OutlineOffset, "2px"))
}

fn newsletter() -> CssStylesheet {
    CssStylesheet::new()
        .push(newsletter_form())
        .push(field())
        .push(field_label())
        .push(field_input())
        .push(field_input_focus())
}

fn site_footer_rule() -> CssRule {
    CssRule::new()
        .selectors_list(".site-footer")
        .push_property((BorderTop, "1px solid"))
        .push_property((BorderColor, CssFnVar::from(ColorBorder)))
        .push_property((PaddingBlock, "2rem"))
}

fn footer_inner() -> CssRule {
    CssRule::new()
        .selectors_list(".footer-inner")
        .push_property((Display, "flex"))
        .push_property((JustifyContent, "space-between"))
        .push_property((AlignItems, "center"))
        .push_property((FlexWrap, "wrap"))
        .push_property((Gap, "0.75rem"))
}

fn footer_inner_address() -> CssRule {
    CssRule::new()
        .selectors_list(".footer-inner address")
        .push_property((FontStyle, "normal"))
        .push_property((FontSize, "0.875rem"))
}

fn site_footer() -> CssStylesheet {
    CssStylesheet::new()
        .push(site_footer_rule())
        .push(footer_inner())
        .push(footer_inner_address())
}
