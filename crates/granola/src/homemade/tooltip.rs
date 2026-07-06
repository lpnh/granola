use askama::Template;
use std::borrow::Cow;

use crate::{homemade::*, prelude::*, recipes::*};

/// The homemade recipe for the `tooltip` component.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let tip = HtmlButton::from(Tip).content("i");
///
/// let tooltip = HtmlSpan::from(Tooltip)
///     .with_id("tip_id", tip)
///     .text("Roll 2d10.")
///     .placement(Placement::Bottom);
///
/// assert_eq!(
///     tooltip.bake(),
///     r#"<span class="tooltip tooltip-bottom"><button class="tip" type="button" aria-describedby="tip_id">i</button><span class="tip-bubble" id="tip_id" role="tooltip">Roll 2d10.</span></span>"#
/// );
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let stylesheet = CssStylesheet::from(Tooltip);
///
/// assert_eq!(
///     stylesheet.bake_pretty(),
///     r#".tooltip {
///   display: inline flex;
///   anchor-scope: all;
///   anchor-name: --tip;
/// }
/// .tip {
///   display: inline flex;
///   align-items: center;
///   justify-content: center;
///   width: 1.15rem;
///   height: 1.15rem;
///   padding: 0;
///   border: 1px solid;
///   border-color: var(--color-border);
///   border-radius: 50%;
///   background-color: var(--color-surface);
///   color: color-mix(in oklab, var(--color-text) 60%, #0000);
///   font-size: 0.7rem;
///   font-style: italic;
///   font-weight: 700;
///   line-height: 1;
///   cursor: help;
/// }
/// .tip:hover, .tip:focus-visible {
///   color: var(--color-text);
///   border-color: var(--color-primary);
/// }
/// .tip-bubble {
///   display: none;
///   opacity: 0;
///   transition: opacity 160ms ease, display 160ms allow-discrete;
///   position: fixed;
///   position-anchor: --tip;
///   anchor-name: --tip-bubble;
///   position-area: top;
///   margin-bottom: 0.5rem;
///   position-try-fallbacks: flip-block;
///   width: max-content;
///   max-width: 20rem;
///   padding: 0.25rem 0.5rem;
///   border-radius: 0.5rem;
///   background-color: var(--color-text);
///   color: var(--color-background);
///   font-size: 0.8rem;
///   font-style: normal;
///   font-weight: 400;
///   line-height: 1.25;
///   text-align: center;
///   white-space: normal;
///   pointer-events: none;
///   z-index: 2;
/// }
/// .tip-bubble::before {
///   content: "";
///   z-index: -1;
///   position: fixed;
///   position-anchor: --tip;
///   position-area: top;
///   width: 0.6rem;
///   height: 0.6rem;
///   background-color: var(--color-text);
///   transform: rotate(45deg);
///   margin-bottom: 0.35rem;
///   margin-top: calc(anchor-size(--tip-bubble block) - 0.3rem);
///   position-try-fallbacks: flip-block;
/// }
/// .tooltip:hover .tip-bubble, .tooltip:has(:focus-visible) .tip-bubble {
///   display: block;
///   opacity: 1;
/// }
/// @starting-style {
///   .tooltip:hover .tip-bubble, .tooltip:has(:focus-visible) .tip-bubble {
///     opacity: 0;
///   }
/// }
/// .tooltip-top .tip-bubble {
///   position-area: top;
///   margin-bottom: 0.5rem;
/// }
/// .tooltip-bottom .tip-bubble {
///   position-area: bottom;
///   margin-bottom: 0;
///   margin-top: 0.5rem;
/// }
/// .tooltip-left .tip-bubble {
///   position-area: left;
///   margin-bottom: 0;
///   margin-right: 0.5rem;
///   position-try-fallbacks: flip-inline;
/// }
/// .tooltip-right .tip-bubble {
///   position-area: right;
///   margin-bottom: 0;
///   margin-left: 0.5rem;
///   position-try-fallbacks: flip-inline;
/// }
/// .tooltip-top .tip-bubble::before {
///   position-area: top;
///   margin-bottom: 0.35rem;
///   margin-top: calc(anchor-size(--tip-bubble block) - 0.3rem);
/// }
/// .tooltip-bottom .tip-bubble::before {
///   position-area: bottom;
///   margin-top: 0.35rem;
///   margin-bottom: calc(anchor-size(--tip-bubble block) - 0.3rem);
/// }
/// .tooltip-left .tip-bubble::before {
///   position-area: left;
///   margin-top: 0;
///   margin-bottom: 0;
///   margin-right: 0.35rem;
///   margin-left: calc(anchor-size(--tip-bubble inline) - 0.3rem);
///   position-try-fallbacks: flip-inline;
/// }
/// .tooltip-right .tip-bubble::before {
///   position-area: right;
///   margin-top: 0;
///   margin-bottom: 0;
///   margin-left: 0.35rem;
///   margin-right: calc(anchor-size(--tip-bubble inline) - 0.3rem);
///   position-try-fallbacks: flip-inline;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tooltip;

/// The [`Tooltip`] content.
///
/// # Askama template
///
/// ```askama
/// {{ element }}{{ bubble }}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TooltipContent {
    element: Bake,
    bubble: HtmlSpan<TipBubble>,
}

impl TooltipContent {
    pub fn new(
        mut self,
        id: impl Into<Cow<'static, str>>,
        element: impl Into<Bake> + HasGlobalAriaAttrs,
    ) -> Self {
        let cloned_id = id.into().to_string();
        self.element = element.aria_describedby(cloned_id.clone()).into();
        self.bubble = self.bubble.id(cloned_id.clone());
        self
    }
}

impl SpanRecipe for Tooltip {
    recipe_boilerplate!(SpanRecipe, TooltipContent);

    fn content_recipe() -> Self::Content {
        Self::Content {
            bubble: HtmlSpan::from(TipBubble),
            ..Default::default()
        }
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("tooltip")
    }
}

impl DivRecipe for Tooltip {
    recipe_boilerplate!(DivRecipe, TooltipContent);

    fn content_recipe() -> Self::Content {
        Self::Content {
            bubble: HtmlSpan::from(TipBubble),
            ..Default::default()
        }
    }

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("tooltip")
    }
}

impl HtmlSpan<Tooltip> {
    pub fn with_id(
        mut self,
        id: impl Into<Cow<'static, str>>,
        element: impl Into<Bake> + HasGlobalAriaAttrs,
    ) -> Self {
        let id_str = id.into();
        self.content = self.content.new(id_str.clone(), element);
        self
    }

    pub fn text(mut self, text: impl Into<Bake>) -> Self {
        self.content.bubble = self.content.bubble.content(text);
        self
    }
}

impl HtmlDiv<Tooltip> {
    pub fn with_id(
        self,
        id: impl Into<Cow<'static, str>>,
        element: impl Into<Bake> + HasGlobalAriaAttrs,
    ) -> Self {
        Self::from(Tooltip).content.new(id.into(), element);
        self
    }

    pub fn text(mut self, text: impl Into<Bake>) -> Self {
        self.content.bubble = self.content.bubble.content(text);
        self
    }
}

impl HasPlacement for HtmlSpan<Tooltip> {
    const COMPONENT_PREFIX: &'static str = "tooltip";
}
impl HasPlacement for HtmlDiv<Tooltip> {
    const COMPONENT_PREFIX: &'static str = "tooltip";
}

impl StylesheetRecipe for Tooltip {
    fn statements_recipe() -> Bake {
        bake_ws![
            CssRule::from(Tooltip),
            CssRule::from(Tip),
            tip_trigger_hover(),
            CssRule::from(TipBubble),
            bubble_tail(),
            tip_bubble_reveal(),
            tip_bubble_starting_style(),
            tip_bubble_placements(),
            tip_bubble_tail_placements(),
        ]
    }
}

fn tip_bubble_placements() -> Bake {
    let top = CssRule::new()
        .selectors_list(".tooltip-top .tip-bubble")
        .declarations_block(bake_ws![
            CssPositionArea::new().content("top"),
            CssMarginBottom::new().content("0.5rem"),
        ]);

    let bottom = CssRule::new()
        .selectors_list(".tooltip-bottom .tip-bubble")
        .declarations_block(bake_ws![
            CssPositionArea::new().content("bottom"),
            CssMarginBottom::new().content("0"),
            CssMarginTop::new().content("0.5rem"),
        ]);

    let left = CssRule::new()
        .selectors_list(".tooltip-left .tip-bubble")
        .declarations_block(bake_ws![
            CssPositionArea::new().content("left"),
            CssMarginBottom::new().content("0"),
            CssMarginRight::new().content("0.5rem"),
            CssPositionTryFallbacks::new().content("flip-inline"),
        ]);

    let right = CssRule::new()
        .selectors_list(".tooltip-right .tip-bubble")
        .declarations_block(bake_ws![
            CssPositionArea::new().content("right"),
            CssMarginBottom::new().content("0"),
            CssMarginLeft::new().content("0.5rem"),
            CssPositionTryFallbacks::new().content("flip-inline"),
        ]);

    bake![top, bottom, left, right]
}

fn tip_bubble_tail_placements() -> Bake {
    let top = CssRule::new()
        .selectors_list(".tooltip-top .tip-bubble::before")
        .declarations_block(bake_ws![
            CssPositionArea::new().content("top"),
            CssMarginBottom::new().content("0.35rem"),
            CssMarginTop::new().content(calc_anchor_size_block()),
        ]);

    let bottom = CssRule::new()
        .selectors_list(".tooltip-bottom .tip-bubble::before")
        .push_property(CssPositionArea::new().content("bottom"))
        .push_property(CssMarginTop::new().content("0.35rem"))
        .push_property(CssMarginBottom::new().content(calc_anchor_size_block()));

    let left = CssRule::new()
        .selectors_list(".tooltip-left .tip-bubble::before")
        .declarations_block(bake_ws![
            CssPositionArea::new().content("left"),
            CssMarginTop::new().content("0"),
            CssMarginBottom::new().content("0"),
            CssMarginRight::new().content("0.35rem"),
            CssMarginLeft::new().content(calc_anchor_size_inline()),
            CssPositionTryFallbacks::new().content("flip-inline"),
        ]);

    let right = CssRule::new()
        .selectors_list(".tooltip-right .tip-bubble::before")
        .declarations_block(bake_ws![
            CssPositionArea::new().content("right"),
            CssMarginTop::new().content("0"),
            CssMarginBottom::new().content("0"),
            CssMarginLeft::new().content("0.35rem"),
            CssMarginRight::new().content(calc_anchor_size_inline()),
            CssPositionTryFallbacks::new().content("flip-inline"),
        ]);

    bake![top, bottom, left, right]
}

impl RuleRecipe for Tooltip {
    fn selectors_list_recipe() -> Bake {
        ".tooltip".into()
    }

    fn declarations_block_recipe() -> Bake {
        bake_ws![
            CssDisplay::new().fold_in("inline").fold_in("flex"),
            CssAnchorScope::new().content("all"),
            CssAnchorName::new().content("--tip"),
        ]
    }
}

/// The homemade recipe for the `tip` button.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let trigger = HtmlButton::from(Tip).content("i");
///
/// assert_eq!(
///     trigger.bake(),
///     r#"<button class="tip" type="button">i</button>"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tip;

impl ButtonRecipe for Tip {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("tip")
    }

    fn specific_attrs_recipe() -> ButtonAttrs {
        ButtonAttrs::default().button_type(ButtonType::Button)
    }
}

impl RuleRecipe for Tip {
    fn selectors_list_recipe() -> Bake {
        ".tip".into()
    }

    fn declarations_block_recipe() -> Bake {
        bake_ws![
            CssDisplay::new().fold_in("inline").fold_in("flex"),
            CssAlignItems::from(Center),
            CssJustifyContent::from(Center),
            CssWidth::new().content("1.15rem"),
            CssHeight::new().content("1.15rem"),
            CssPadding::new().content("0"),
            CssBorder::new().content("1px solid"),
            CssBorderColor::new().content(CssFnVar::from(ColorBorder)),
            CssBorderRadius::new().content("50%"),
            CssBackgroundColor::new().content(CssFnVar::from(ColorSurface)),
            CssColor::new().content("color-mix(in oklab, var(--color-text) 60%, #0000)"),
            CssFontSize::new().content("0.7rem"),
            CssFontStyle::new().content("italic"),
            CssFontWeight::new().content("700"),
            CssLineHeight::new().content("1"),
            CssCursor::new().content("help"),
        ]
    }
}

fn tip_trigger_hover() -> CssRule {
    CssRule::new()
        .push_selector(".tip:hover")
        .push_selector(".tip:focus-visible")
        .push_property(CssColor::new().content(CssFnVar::from(ColorText)))
        .push_property(CssBorderColor::new().content(CssFnVar::from(ColorPrimary)))
}

/// The homemade recipe for the `tip-bubble` element.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let bubble = HtmlSpan::from(TipBubble).content("Roll 2d10.");
///
/// assert_eq!(
///     bubble.bake(),
///     r#"<span class="tip-bubble" role="tooltip">Roll 2d10.</span>"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct TipBubble;

impl SpanRecipe for TipBubble {
    recipe_boilerplate!(SpanRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("tip-bubble").role("tooltip")
    }
}

impl RuleRecipe for TipBubble {
    fn selectors_list_recipe() -> Bake {
        ".tip-bubble".into()
    }

    fn declarations_block_recipe() -> Bake {
        bake_ws![
            CssDisplay::from(None),
            CssOpacity::new().content("0"),
            CssTransition::new().content("opacity 160ms ease, display 160ms allow-discrete"),
            CssPosition::new().content("fixed"),
            CssPositionAnchor::new().content("--tip"),
            CssAnchorName::new().content("--tip-bubble"),
            CssPositionArea::new().content("top"),
            CssMarginBottom::new().content("0.5rem"),
            CssPositionTryFallbacks::new().content("flip-block"),
            CssWidth::new().content("max-content"),
            CssMaxWidth::new().content("20rem"),
            CssPadding::new().content("0.25rem 0.5rem"),
            CssBorderRadius::new().content("0.5rem"),
            CssBackgroundColor::new().content(CssFnVar::from(ColorText)),
            CssColor::new().content(CssFnVar::from(ColorBackground)),
            CssFontSize::new().content("0.8rem"),
            CssFontStyle::new().content("normal"),
            CssFontWeight::new().content("400"),
            CssLineHeight::new().content("1.25"),
            CssTextAlign::new().content("center"),
            CssWhiteSpace::new().content("normal"),
            CssPointerEvents::new().content("none"),
            CssZIndex::new().content("2"),
        ]
    }
}

fn bubble_tail() -> CssRule {
    let declarations_block = bake_ws![
        CssContent::new().content(r#""""#),
        CssZIndex::new().content("-1"),
        CssPosition::new().content("fixed"),
        CssPositionAnchor::new().content("--tip"),
        CssPositionArea::new().content("top"),
        CssWidth::new().content("0.6rem"),
        CssHeight::new().content("0.6rem"),
        CssBackgroundColor::new().content(CssFnVar::from(ColorText)),
        CssTransform::new().content("rotate(45deg)"),
        CssMarginBottom::new().content("0.35rem"),
        CssMarginTop::new().content(calc_anchor_size_block()),
        CssPositionTryFallbacks::new().content("flip-block"),
    ];

    CssRule::new()
        .selectors_list(".tip-bubble::before")
        .declarations_block(declarations_block)
}

fn tip_bubble_reveal() -> CssRule {
    CssRule::new()
        .push_selector(".tooltip:hover .tip-bubble")
        .push_selector(".tooltip:has(:focus-visible) .tip-bubble")
        .push_property(CssDisplay::from(Block))
        .push_property(CssOpacity::new().content("1"))
}

fn tip_bubble_starting_style() -> Bake {
    let revealed = CssRule::new()
        .selectors_list(bake_comma![
            ".tooltip:hover .tip-bubble",
            ".tooltip:has(:focus-visible) .tip-bubble",
        ])
        .declarations_block(CssOpacity::new().content("0"));

    CssAtRule::new()
        .identifier("starting-style")
        .block(revealed)
        .into()
}

/// The [`Placement`] modifier for the components.
///
/// Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let tooltip = HtmlSpan::from(Tooltip).placement(Placement::Bottom);
///
/// assert_eq!(
///     tooltip.bake(),
///     r#"<span class="tooltip tooltip-bottom"><span class="tip-bubble" role="tooltip"></span></span>"#
/// );
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let tooltip = HtmlDiv::from(Tooltip).placement(Placement::Left);
///
/// assert_eq!(
///     tooltip.bake(),
///     r#"<div class="tooltip tooltip-left"><span class="tip-bubble" role="tooltip"></span></div>"#
/// );
/// ```
pub trait HasPlacement: HasGlobalAttrs + Sized {
    const COMPONENT_PREFIX: &'static str;

    fn placement(self, placement: Placement) -> Self {
        self.class(format!("{}-{placement}", Self::COMPONENT_PREFIX))
    }
}

#[derive(strum::Display, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum Placement {
    Top,
    Bottom,
    Left,
    Right,
}

fn calc_anchor_size_block() -> CssFnCalc {
    CssFnCalc::new().subtract(CssFnAnchorSize::new().value("--tip-bubble block"), "0.3rem")
}

fn calc_anchor_size_inline() -> CssFnCalc {
    CssFnCalc::new().subtract(
        CssFnAnchorSize::new().value("--tip-bubble inline"),
        "0.3rem",
    )
}
