use granola::{homemade::*, macros::*, prelude::*, recipes::*};

pub fn style() -> CssStylesheet {
    CssStylesheet::from(OatsAndEndsRecipes).bake_recipe()
}

#[derive(Default, Debug, Clone, PartialEq)]
struct OatsAndEndsRecipes;

impl StylesheetRecipe for OatsAndEndsRecipes {
    recipe_boilerplate!(StylesheetRecipe, CssStylesheet);

    fn content_recipe() -> Self::Content {
        stylesheet![
            CssStylesheet::from(AndyBell),
            CssStylesheet::from(Palette),
            CssStylesheet::from(PaletteDark),
            CssStylesheet::from(Btn),
            CssStylesheet::from(Tooltip),
            CssStylesheet::from(Typography),
            CssStylesheet::from(Layout),
            CssStylesheet::from(SkipLink),
            CssStylesheet::from(SiteHeader),
            CssStylesheet::from(Hero),
            CssStylesheet::from(Menu),
            CssStylesheet::from(Hours),
            CssStylesheet::from(Visit),
            CssStylesheet::from(Newsletter),
            CssStylesheet::from(SiteFooter),
        ]
    }
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

#[derive(Default, Debug, Clone, PartialEq)]
struct Palette;

impl StylesheetRecipe for Palette {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        CssRule::from(Palette).into()
    }
}

impl RuleRecipe for Palette {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ":root".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((ColorBackground, "#fbf4e8")),
            CssDeclaration::from((ColorSurface, "#ffffff")),
            CssDeclaration::from((ColorBorder, "#e4d3b6")),
            CssDeclaration::from((ColorText, "#2c1c12")),
            CssDeclaration::from((ColorPrimary, "#96491f")),
            CssDeclaration::from((ColorPrimaryText, "#fffaf5")),
            CssDeclaration::from((ColorError, "#9a2f22")),
            CssDeclaration::from((ColorSuccess, "#2f6b46")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct PaletteDark;

impl StylesheetRecipe for PaletteDark {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        CssAtRule::from(PaletteDark).into()
    }
}

impl AtRuleRecipe for PaletteDark {
    fn identifier_recipe() -> Bake {
        "media".into()
    }

    fn rule_recipe() -> Bake {
        "(prefers-color-scheme: dark)".into()
    }

    fn block_recipe() -> Option<Bake> {
        Some(CssRule::from(PaletteDarkRoot).into())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct PaletteDarkRoot;

impl RuleRecipe for PaletteDarkRoot {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ":root".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((ColorBackground, "#1c140d")),
            CssDeclaration::from((ColorSurface, "#271c13")),
            CssDeclaration::from((ColorBorder, "#4a3823")),
            CssDeclaration::from((ColorText, "#f3e7d6")),
            CssDeclaration::from((ColorPrimary, "#e2905c")),
            CssDeclaration::from((ColorPrimaryText, "#221207")),
            CssDeclaration::from((ColorError, "#e2897d")),
            CssDeclaration::from((ColorSuccess, "#83c99b")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Typography;

impl StylesheetRecipe for Typography {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(Body),
            CssRule::from((
                Headings,
                bake_ws![
                    CssDeclaration::from((
                        FontFamily,
                        "Georgia, 'Iowan Old Style', ui-serif, serif"
                    )),
                    CssDeclaration::from((FontWeight, "600")),
                ]
            )),
            CssRule::from(H1),
            CssRule::from(H2),
            CssRule::from(H3),
            CssRule::from(Lede),
            CssRule::from(Note),
            CssRule::from(BodyLinks),
            CssRule::from(Selection),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Body;

impl RuleRecipe for Body {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "body".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((FontFamily, "system-ui, sans-serif")),
            CssDeclaration::from((Color, CssFnVar::from(ColorText))),
            CssDeclaration::from((BackgroundColor, CssFnVar::from(ColorBackground))),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct H1;

impl RuleRecipe for H1 {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "h1".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((
            FontSize,
            CssFnClamp::new()
                .min("2rem")
                .val("1.5rem + 2vw")
                .max("3.25rem"),
        ))
        .into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct H2;

impl RuleRecipe for H2 {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "h2".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((FontSize, "1.75rem")).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct H3;

impl RuleRecipe for H3 {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "h3".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((FontSize, "1.25rem")).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Lede;

impl RuleRecipe for Lede {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".lede".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((MaxWidth, "38rem")),
            CssDeclaration::from((FontSize, "1.125rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Note;

impl RuleRecipe for Note {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".note".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((MaxWidth, "38rem")),
            CssDeclaration::from((Color, muted_text())),
            CssDeclaration::from((FontSize, "0.9rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct BodyLinks;

impl RuleRecipe for BodyLinks {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "main a:not(.btn), .footer-inner a".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Color, CssFnVar::from(ColorPrimary))),
            CssDeclaration::from((TextUnderlineOffset, "0.15em")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Selection;

impl RuleRecipe for Selection {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "::selection".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((BackgroundColor, selection_tint())).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Layout;

impl StylesheetRecipe for Layout {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(Wrap),
            CssRule::from(SectionPadding),
            CssAtRule::from(ReducedMotion),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Wrap;

impl RuleRecipe for Wrap {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".wrap".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((MaxWidth, "68rem")),
            CssDeclaration::from((MarginInline, "auto")),
            CssDeclaration::from((
                PaddingInline,
                CssFnClamp::new().min("1rem").val("4vw").max("2.5rem"),
            )),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SectionPadding;

impl RuleRecipe for SectionPadding {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".section".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((PaddingBlock, "3rem")).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct ReducedMotion;

impl AtRuleRecipe for ReducedMotion {
    fn identifier_recipe() -> Bake {
        "media".into()
    }

    fn rule_recipe() -> Bake {
        "(prefers-reduced-motion: no-preference)".into()
    }

    fn block_recipe() -> Option<Bake> {
        Some(CssRule::from(ScrollBehaviorSmooth).into())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct ScrollBehaviorSmooth;

impl RuleRecipe for ScrollBehaviorSmooth {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "html".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((ScrollBehavior, "smooth")).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SkipLink;

impl StylesheetRecipe for SkipLink {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![CssRule::from(SkipLinkRule), CssRule::from(SkipLinkFocus)]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SkipLinkRule;

impl RuleRecipe for SkipLinkRule {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".skip-link".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Position, "absolute")),
            CssDeclaration::from((Top, "-3rem")),
            CssDeclaration::from((Left, "1rem")),
            CssDeclaration::from((Padding, "0.5em 1em")),
            CssDeclaration::from((BorderRadius, "0.35em")),
            CssDeclaration::from((BackgroundColor, CssFnVar::from(ColorText))),
            CssDeclaration::from((Color, CssFnVar::from(ColorBackground))),
            CssDeclaration::from((TextDecoration, "none")),
            CssDeclaration::from((ZIndex, "10")),
            CssDeclaration::from((Transition, "top 150ms ease")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SkipLinkFocus;

impl RuleRecipe for SkipLinkFocus {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".skip-link:focus-visible".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((Top, "1rem")).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SiteHeader;

impl StylesheetRecipe for SiteHeader {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(SiteHeaderRule),
            CssRule::from(SiteNav),
            CssRule::from(Brand),
            CssRule::from(NavLinks),
            CssRule::from(NavLinksAnchor),
            CssRule::from(NavLinksAnchorHover),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SiteHeaderRule;

impl RuleRecipe for SiteHeaderRule {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".site-header".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((BorderBottom, "1px solid")),
            CssDeclaration::from((BorderColor, CssFnVar::from(ColorBorder))),
            CssDeclaration::from((Position, "sticky")),
            CssDeclaration::from((Top, "0")),
            CssDeclaration::from((ZIndex, "5")),
            CssDeclaration::from((BackgroundColor, CssFnVar::from(ColorBackground))),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SiteNav;

impl RuleRecipe for SiteNav {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".site-nav".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((AlignItems, "center")),
            CssDeclaration::from((JustifyContent, "space-between")),
            CssDeclaration::from((Gap, "1rem")),
            CssDeclaration::from((PaddingBlock, "1rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Brand;

impl RuleRecipe for Brand {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".brand".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((FontFamily, "Georgia, 'Iowan Old Style', ui-serif, serif")),
            CssDeclaration::from((FontWeight, "700")),
            CssDeclaration::from((FontSize, "1.25rem")),
            CssDeclaration::from((Color, CssFnVar::from(ColorText))),
            CssDeclaration::from((TextDecoration, "none")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct NavLinks;

impl RuleRecipe for NavLinks {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".nav-links".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((AlignItems, "center")),
            CssDeclaration::from((Gap, "1.5rem")),
            CssDeclaration::from((FlexWrap, "wrap")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct NavLinksAnchor;

impl RuleRecipe for NavLinksAnchor {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".nav-links a:not(.btn)".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Color, CssFnVar::from(ColorText))),
            CssDeclaration::from((FontWeight, "500")),
            CssDeclaration::from((TextDecoration, "none")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct NavLinksAnchorHover;

impl RuleRecipe for NavLinksAnchorHover {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".nav-links a:not(.btn):hover".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((TextDecoration, "underline")).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Hero;

impl StylesheetRecipe for Hero {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(HeroRule),
            CssRule::from(HeroLede),
            CssRule::from(HeroActions),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct HeroRule;

impl RuleRecipe for HeroRule {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".hero".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((PaddingBlock, "4rem 3rem")),
            CssDeclaration::from((TextAlign, "center")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct HeroLede;

impl RuleRecipe for HeroLede {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".hero .lede".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((MaxWidth, "none")).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct HeroActions;

impl RuleRecipe for HeroActions {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".hero-actions".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((JustifyContent, "center")),
            CssDeclaration::from((Gap, "1rem")),
            CssDeclaration::from((FlexWrap, "wrap")),
            CssDeclaration::from((MarginTop, "2rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Menu;

impl StylesheetRecipe for Menu {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(MenuGroups),
            CssAtRule::from(MenuGroupsWide),
            CssRule::from(MenuGroupHeading),
            CssRule::from(MenuList),
            CssRule::from(MenuItem),
            CssRule::from(MenuItemName),
            CssRule::from(MenuItemDesc),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct MenuGroups;

impl RuleRecipe for MenuGroups {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".menu-groups".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "grid")),
            CssDeclaration::from((Gap, "3rem")),
            CssDeclaration::from((MarginTop, "2rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct MenuGroupsWide;

impl AtRuleRecipe for MenuGroupsWide {
    fn identifier_recipe() -> Bake {
        "media".into()
    }

    fn rule_recipe() -> Bake {
        "(min-width: 720px)".into()
    }

    fn block_recipe() -> Option<Bake> {
        Some(CssRule::from(MenuGroupsColumns).into())
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct MenuGroupsColumns;

impl RuleRecipe for MenuGroupsColumns {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".menu-groups".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from((GridTemplateColumns, "1fr 1fr")).into()
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct MenuGroupHeading;

impl RuleRecipe for MenuGroupHeading {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".menu-group h3".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((BorderBottom, "1px solid")),
            CssDeclaration::from((BorderColor, CssFnVar::from(ColorBorder))),
            CssDeclaration::from((Padding, "0 0 0.5rem")),
            CssDeclaration::from((MarginBottom, "1rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct MenuList;

impl RuleRecipe for MenuList {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".menu-list".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((FlexDirection, "column")),
            CssDeclaration::from((Gap, "1.25rem")),
            CssDeclaration::from((Margin, "0")),
            CssDeclaration::from((Padding, "0")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct MenuItem;

impl RuleRecipe for MenuItem {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".menu-item".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((FlexWrap, "wrap")),
            CssDeclaration::from((JustifyContent, "space-between")),
            CssDeclaration::from((AlignItems, "baseline")),
            CssDeclaration::from((Gap, "0.25rem 1rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct MenuItemName;

impl RuleRecipe for MenuItemName {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".menu-item-name".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "inline-flex")),
            CssDeclaration::from((AlignItems, "center")),
            CssDeclaration::from((Gap, "0.4rem")),
            CssDeclaration::from((FontWeight, "600")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct MenuItemDesc;

impl RuleRecipe for MenuItemDesc {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".menu-item-desc".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((FlexBasis, "100%")),
            CssDeclaration::from((Color, muted_text())),
            CssDeclaration::from((FontSize, "0.9rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Hours;

impl StylesheetRecipe for Hours {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(HoursTable),
            CssRule::from(HoursCaption),
            CssRule::from(HoursCells),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct HoursTable;

impl RuleRecipe for HoursTable {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "table".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Width, "100%")),
            CssDeclaration::from((BorderCollapse, "collapse")),
            CssDeclaration::from((MarginTop, "1.5rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct HoursCaption;

impl RuleRecipe for HoursCaption {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "caption".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((TextAlign, "left")),
            CssDeclaration::from((FontWeight, "600")),
            CssDeclaration::from((MarginBottom, "0.75rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct HoursCells;

impl RuleRecipe for HoursCells {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma!["th", "td"]
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((TextAlign, "left")),
            CssDeclaration::from((Padding, "0.6rem 1rem 0.6rem 0")),
            CssDeclaration::from((BorderBottom, "1px solid")),
            CssDeclaration::from((BorderColor, CssFnVar::from(ColorBorder))),
            CssDeclaration::from((FontVariantNumeric, "tabular-nums")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Visit;

impl StylesheetRecipe for Visit {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![CssRule::from(VisitAddress), CssRule::from(VisitActions)]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct VisitAddress;

impl RuleRecipe for VisitAddress {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "#visit address".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((FontStyle, "normal")),
            CssDeclaration::from((LineHeight, "1.7")),
            CssDeclaration::from((MarginTop, "1rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct VisitActions;

impl RuleRecipe for VisitActions {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".visit-actions".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((Gap, "1rem")),
            CssDeclaration::from((FlexWrap, "wrap")),
            CssDeclaration::from((MarginTop, "1.5rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Newsletter;

impl StylesheetRecipe for Newsletter {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(NewsletterForm),
            CssRule::from(Field),
            CssRule::from(FieldLabel),
            CssRule::from(FieldInput),
            CssRule::from(FieldInputFocus),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct NewsletterForm;

impl RuleRecipe for NewsletterForm {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "#newsletter form".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((FlexWrap, "wrap")),
            CssDeclaration::from((AlignItems, "flex-end")),
            CssDeclaration::from((Gap, "1rem")),
            CssDeclaration::from((MarginTop, "1.5rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct Field;

impl RuleRecipe for Field {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".field".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((FlexDirection, "column")),
            CssDeclaration::from((Gap, "0.35rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct FieldLabel;

impl RuleRecipe for FieldLabel {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".field label".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((FontSize, "0.875rem")),
            CssDeclaration::from((FontWeight, "500")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct FieldInput;

impl RuleRecipe for FieldInput {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".field input".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Padding, "0.6em 0.8em")),
            CssDeclaration::from((Border, "1px solid")),
            CssDeclaration::from((BorderColor, CssFnVar::from(ColorBorder))),
            CssDeclaration::from((BorderRadius, "0.5em")),
            CssDeclaration::from((BackgroundColor, CssFnVar::from(ColorSurface))),
            CssDeclaration::from((Color, CssFnVar::from(ColorText))),
            CssDeclaration::from((FontSize, "1rem")),
            CssDeclaration::from((MinWidth, "16rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct FieldInputFocus;

impl RuleRecipe for FieldInputFocus {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".field input:focus-visible".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((OutlineWidth, "2px")),
            CssDeclaration::from((OutlineStyle, "solid")),
            CssDeclaration::from((OutlineColor, CssFnVar::from(ColorPrimary))),
            CssDeclaration::from((OutlineOffset, "2px")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SiteFooter;

impl StylesheetRecipe for SiteFooter {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(SiteFooterRule),
            CssRule::from(FooterInner),
            CssRule::from(FooterInnerAddress),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct SiteFooterRule;

impl RuleRecipe for SiteFooterRule {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".site-footer".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((BorderTop, "1px solid")),
            CssDeclaration::from((BorderColor, CssFnVar::from(ColorBorder))),
            CssDeclaration::from((PaddingBlock, "2rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct FooterInner;

impl RuleRecipe for FooterInner {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".footer-inner".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((Display, "flex")),
            CssDeclaration::from((JustifyContent, "space-between")),
            CssDeclaration::from((AlignItems, "center")),
            CssDeclaration::from((FlexWrap, "wrap")),
            CssDeclaration::from((Gap, "0.75rem")),
        ]
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
struct FooterInnerAddress;

impl RuleRecipe for FooterInnerAddress {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".footer-inner address".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from((FontStyle, "normal")),
            CssDeclaration::from((FontSize, "0.875rem")),
        ]
    }
}
