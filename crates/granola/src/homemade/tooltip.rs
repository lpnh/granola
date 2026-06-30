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
    element: Cow<'static, str>,
    bubble: HtmlSpan<TipBubble>,
}

impl TooltipContent {
    pub fn new(
        mut self,
        id: impl Into<Cow<'static, str>>,
        element: impl Into<Cow<'static, str>> + HasGlobalAriaAttrs,
    ) -> Self {
        let cloned_id = id.into().to_string();
        self.element = element.aria_describedby(cloned_id.clone()).into();
        self.bubble = self.bubble.id(cloned_id.clone());
        self
    }
}

impl SpanRecipe for Tooltip {
    type Content = TooltipContent;

    fn bake_content(content: Self::Content) -> Cow<'static, str> {
        content.into()
    }

    fn content_recipe(content: &mut Self::Content) {
        content.bubble = HtmlSpan::from(TipBubble);
    }

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("tooltip");
    }
}

impl DivRecipe for Tooltip {
    type Content = TooltipContent;

    fn bake_content(content: Self::Content) -> Cow<'static, str> {
        content.into()
    }

    fn content_recipe(content: &mut Self::Content) {
        content.bubble = HtmlSpan::from(TipBubble);
    }

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("tooltip");
    }
}

impl HtmlSpan<Tooltip> {
    pub fn with_id(
        mut self,
        id: impl Into<Cow<'static, str>>,
        element: impl Into<Cow<'static, str>> + HasGlobalAriaAttrs,
    ) -> Self {
        let id_str = id.into();
        self.content = self.content.new(id_str.clone(), element);
        self
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.content.bubble = self.content.bubble.content(text);
        self
    }
}

impl HtmlDiv<Tooltip> {
    pub fn with_id(
        self,
        id: impl Into<Cow<'static, str>>,
        element: impl Into<Cow<'static, str>> + HasGlobalAriaAttrs,
    ) -> Self {
        Self::from(Tooltip).content.new(id.into(), element);
        self
    }

    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
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
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([
            CssRule::from(Tooltip).into(),
            CssRule::from(Tip).into(),
            tip_trigger_hover().into(),
            CssRule::from(TipBubble).into(),
            bubble_tail().into(),
            tip_bubble_reveal().into(),
            tip_bubble_starting_style(),
        ]);
        statements.extend(tip_bubble_placements());
        statements.extend(tip_bubble_tail_placements());
    }
}

fn tip_bubble_placements() -> Vec<CssStatement> {
    let top = CssRule::new()
        .selectors_list(".tooltip-top .tip-bubble")
        .declarations_block([
            CssDeclaration::from(CssPositionArea::new().content("top")),
            CssMarginBottom::new().content("0.5rem").into(),
        ]);

    let bottom = CssRule::new()
        .selectors_list(".tooltip-bottom .tip-bubble")
        .declarations_block([
            CssDeclaration::from(CssPositionArea::new().content("bottom")),
            CssMarginBottom::new().content("0").into(),
            CssMarginTop::new().content("0.5rem").into(),
        ]);

    let left = CssRule::new()
        .selectors_list(".tooltip-left .tip-bubble")
        .declarations_block([
            CssDeclaration::from(CssPositionArea::new().content("left")),
            CssMarginBottom::new().content("0").into(),
            CssMarginRight::new().content("0.5rem").into(),
            CssPositionTryFallbacks::new().content("flip-inline").into(),
        ]);

    let right = CssRule::new()
        .selectors_list(".tooltip-right .tip-bubble")
        .declarations_block([
            CssDeclaration::from(CssPositionArea::new().content("right")),
            CssMarginBottom::new().content("0").into(),
            CssMarginLeft::new().content("0.5rem").into(),
            CssPositionTryFallbacks::new().content("flip-inline").into(),
        ]);

    vec![top.into(), bottom.into(), left.into(), right.into()]
}

fn tip_bubble_tail_placements() -> Vec<CssStatement> {
    let top = CssRule::new()
        .selectors_list(".tooltip-top .tip-bubble::before")
        .declarations_block([
            CssDeclaration::from(CssPositionArea::new().content("top")),
            CssMarginBottom::new().content("0.35rem").into(),
            CssMarginTop::new().content(calc_anchor_size_block()).into(),
        ]);

    let bottom = CssRule::new()
        .selectors_list(".tooltip-bottom .tip-bubble::before")
        .push_property(CssPositionArea::new().content("bottom"))
        .push_property(CssMarginTop::new().content("0.35rem"))
        .push_property(CssMarginBottom::new().content(calc_anchor_size_block()));

    let left = CssRule::new()
        .selectors_list(".tooltip-left .tip-bubble::before")
        .declarations_block(
            CssDeclarationsBlock::new().extend([
                CssPositionArea::new().content("left").into(),
                CssMarginTop::new().content("0").into(),
                CssMarginBottom::new().content("0").into(),
                CssMarginRight::new().content("0.35rem").into(),
                CssMarginLeft::new()
                    .content(calc_anchor_size_inline())
                    .into(),
                CssPositionTryFallbacks::new().content("flip-inline").into(),
            ]),
        );

    let right = CssRule::new()
        .selectors_list(".tooltip-right .tip-bubble::before")
        .declarations_block(
            CssDeclarationsBlock::new().extend([
                CssPositionArea::new().content("right").into(),
                CssMarginTop::new().content("0").into(),
                CssMarginBottom::new().content("0").into(),
                CssMarginLeft::new().content("0.35rem").into(),
                CssMarginRight::new()
                    .content(calc_anchor_size_inline())
                    .into(),
                CssPositionTryFallbacks::new().content("flip-inline").into(),
            ]),
        );

    vec![top.into(), bottom.into(), left.into(), right.into()]
}

impl RuleRecipe for Tooltip {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(".tooltip");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssDisplay::new().add_value("inline").add_value("flex").into(),
            CssAnchorScope::new().content("all").into(),
            CssAnchorName::new().content("--tip").into(),
        ]);
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
    recipe_boilerplate!();

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("tip");
    }

    fn specific_attrs_recipe(button_attrs: &mut ButtonAttrs) {
        button_attrs.button_type(ButtonType::Button);
    }
}

impl RuleRecipe for Tip {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(".tip");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssDisplay::new().add_value("inline").add_value("flex").into(),
            CssAlignItems::from(Center).into(),
            CssJustifyContent::from(Center).into(),
            CssWidth::new().content("1.15rem").into(),
            CssHeight::new().content("1.15rem").into(),
            CssPadding::new().content("0").into(),
            CssBorder::new().content("1px solid").into(),
            CssBorderColor::new()
                .content(CssFnVar::from(ColorBorder))
                .into(),
            CssBorderRadius::new().content("50%").into(),
            CssBackgroundColor::new()
                .content(CssFnVar::from(ColorSurface))
                .into(),
            CssColor::new()
                .content("color-mix(in oklab, var(--color-text) 60%, #0000)")
                .into(),
            CssFontSize::new().content("0.7rem").into(),
            CssFontStyle::new().content("italic").into(),
            CssFontWeight::new().content("700").into(),
            CssLineHeight::new().content("1").into(),
            CssCursor::new().content("help").into(),
        ]);
    }
}

fn tip_trigger_hover() -> CssRule {
    let declarations_block: [CssDeclaration; 2] = [
        CssColor::new().content(CssFnVar::from(ColorText)).into(),
        CssBorderColor::new()
            .content(CssFnVar::from(ColorPrimary))
            .into(),
    ];

    CssRule::new()
        .selectors_list([".tip:hover", ".tip:focus-visible"])
        .declarations_block(declarations_block)
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
    recipe_boilerplate!();

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("tip-bubble").role("tooltip");
    }
}

impl RuleRecipe for TipBubble {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(".tip-bubble");
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssDisplay::from(None).into(),
            CssOpacity::new().content("0").into(),
            CssTransition::new()
                .content("opacity 160ms ease, display 160ms allow-discrete")
                .into(),
            CssPosition::new().content("fixed").into(),
            CssPositionAnchor::new().content("--tip").into(),
            CssAnchorName::new().content("--tip-bubble").into(),
            CssPositionArea::new().content("top").into(),
            CssMarginBottom::new().content("0.5rem").into(),
            CssPositionTryFallbacks::new().content("flip-block").into(),
            CssWidth::new().content("max-content").into(),
            CssMaxWidth::new().content("20rem").into(),
            CssPadding::new().content("0.25rem 0.5rem").into(),
            CssBorderRadius::new().content("0.5rem").into(),
            CssBackgroundColor::new()
                .content(CssFnVar::from(ColorText))
                .into(),
            CssColor::new()
                .content(CssFnVar::from(ColorBackground))
                .into(),
            CssFontSize::new().content("0.8rem").into(),
            CssFontStyle::new().content("normal").into(),
            CssFontWeight::new().content("400").into(),
            CssLineHeight::new().content("1.25").into(),
            CssTextAlign::new().content("center").into(),
            CssWhiteSpace::new().content("normal").into(),
            CssPointerEvents::new().content("none").into(),
            CssZIndex::new().content("2").into(),
        ]);
    }
}

fn bubble_tail() -> CssRule {
    let declarations_block = CssDeclarationsBlock::new().extend([
        CssContent::new().content(r#""""#).into(),
        CssZIndex::new().content("-1").into(),
        CssPosition::new().content("fixed").into(),
        CssPositionAnchor::new().content("--tip").into(),
        CssPositionArea::new().content("top").into(),
        CssWidth::new().content("0.6rem").into(),
        CssHeight::new().content("0.6rem").into(),
        CssBackgroundColor::new()
            .content(CssFnVar::from(ColorText))
            .into(),
        CssTransform::new().content("rotate(45deg)").into(),
        CssMarginBottom::new().content("0.35rem").into(),
        CssMarginTop::new().content(calc_anchor_size_block()).into(),
        CssPositionTryFallbacks::new().content("flip-block").into(),
    ]);

    CssRule::new()
        .selectors_list(".tip-bubble::before")
        .declarations_block(declarations_block)
}

fn tip_bubble_reveal() -> CssRule {
    CssRule::new()
        .selectors_list([
            ".tooltip:hover .tip-bubble",
            ".tooltip:has(:focus-visible) .tip-bubble",
        ])
        .push_property(CssDisplay::from(Block))
        .push_property(CssOpacity::new().content("1"))
}

fn tip_bubble_starting_style() -> CssStatement {
    let revealed = CssRule::new()
        .selectors_list([
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
