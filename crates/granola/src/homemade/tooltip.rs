use askama::Template;
use std::borrow::Cow;

use crate::{homemade::*, macros::declarations_block, prelude::*, recipes::*};

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
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
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
        .content(bake_ws![
            CssDeclaration::from(PositionArea).content("top"),
            CssDeclaration::from(MarginBottom).content("0.5rem"),
        ]);

    let bottom = CssRule::new()
        .selectors_list(".tooltip-bottom .tip-bubble")
        .content(bake_ws![
            CssDeclaration::from(PositionArea).content("bottom"),
            CssDeclaration::from(MarginBottom).content("0"),
            CssDeclaration::from(MarginTop).content("0.5rem"),
        ]);

    let left = CssRule::new()
        .selectors_list(".tooltip-left .tip-bubble")
        .content(bake_ws![
            CssDeclaration::from(PositionArea).content("left"),
            CssDeclaration::from(MarginBottom).content("0"),
            CssDeclaration::from(MarginRight).content("0.5rem"),
            CssDeclaration::from(PositionTryFallbacks).content("flip-inline"),
        ]);

    let right = CssRule::new()
        .selectors_list(".tooltip-right .tip-bubble")
        .content(bake_ws![
            CssDeclaration::from(PositionArea).content("right"),
            CssDeclaration::from(MarginBottom).content("0"),
            CssDeclaration::from(MarginLeft).content("0.5rem"),
            CssDeclaration::from(PositionTryFallbacks).content("flip-inline"),
        ]);

    bake![top, bottom, left, right]
}

fn tip_bubble_tail_placements() -> Bake {
    let top = CssRule::new()
        .selectors_list(".tooltip-top .tip-bubble::before")
        .content(bake_ws![
            CssDeclaration::from(PositionArea).content("top"),
            CssDeclaration::from(MarginBottom).content("0.35rem"),
            CssDeclaration::from(MarginTop).content(calc_anchor_size_block()),
        ]);

    let bottom = CssRule::new()
        .selectors_list(".tooltip-bottom .tip-bubble::before")
        .push_property(CssDeclaration::from(PositionArea).content("bottom"))
        .push_property(CssDeclaration::from(MarginTop).content("0.35rem"))
        .push_property(CssDeclaration::from(MarginBottom).content(calc_anchor_size_block()));

    let left = CssRule::new()
        .selectors_list(".tooltip-left .tip-bubble::before")
        .content(bake_ws![
            CssDeclaration::from(PositionArea).content("left"),
            CssDeclaration::from(MarginTop).content("0"),
            CssDeclaration::from(MarginBottom).content("0"),
            CssDeclaration::from(MarginRight).content("0.35rem"),
            CssDeclaration::from(MarginLeft).content(calc_anchor_size_inline()),
            CssDeclaration::from(PositionTryFallbacks).content("flip-inline"),
        ]);

    let right = CssRule::new()
        .selectors_list(".tooltip-right .tip-bubble::before")
        .content(bake_ws![
            CssDeclaration::from(PositionArea).content("right"),
            CssDeclaration::from(MarginTop).content("0"),
            CssDeclaration::from(MarginBottom).content("0"),
            CssDeclaration::from(MarginLeft).content("0.35rem"),
            CssDeclaration::from(MarginRight).content(calc_anchor_size_inline()),
            CssDeclaration::from(PositionTryFallbacks).content("flip-inline"),
        ]);

    bake![top, bottom, left, right]
}

impl RuleRecipe for Tooltip {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".tooltip".into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(Display).content("inline flex"),
            CssDeclaration::from(AnchorScope).content("all"),
            CssDeclaration::from(AnchorName).content("--tip"),
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
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".tip".into()
    }

    fn content_recipe() -> Self::Content {
        declarations_block![
            (Display, "inline flex"),
            (AlignItems, "center"),
            (JustifyContent, "center"),
            (Width, "1.15rem"),
            (Height, "1.15rem"),
            (Padding, "0"),
            (Border, "1px solid"),
            (BorderColor, CssFnVar::from(ColorBorder)),
            (BorderRadius, "50%"),
            (BackgroundColor, CssFnVar::from(ColorSurface)),
            (Color, "color-mix(in oklab, var(--color-text) 60%, #0000)"),
            (FontSize, "0.7rem"),
            (FontStyle, "italic"),
            (FontWeight, "700"),
            (LineHeight, "1"),
            (Cursor, "help"),
        ]
    }
}

fn tip_trigger_hover() -> CssRule {
    CssRule::new()
        .selectors_list(bake_comma![".tip:hover", ".tip:focus-visible"])
        .content(declarations_block![
            (Color, CssFnVar::from(ColorText)),
            (BorderColor, CssFnVar::from(ColorPrimary))
        ])
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
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        ".tip-bubble".into()
    }

    fn content_recipe() -> Self::Content {
        declarations_block![
            (Display, "none"),
            (Opacity, "0"),
            (
                Transition,
                "opacity 160ms ease, display 160ms allow-discrete"
            ),
            (Position, "fixed"),
            (PositionAnchor, "--tip"),
            (AnchorName, "--tip-bubble"),
            (PositionArea, "top"),
            (MarginBottom, "0.5rem"),
            (PositionTryFallbacks, "flip-block"),
            (Width, "max-content"),
            (MaxWidth, "20rem"),
            (Padding, "0.25rem 0.5rem"),
            (BorderRadius, "0.5rem"),
            (BackgroundColor, CssFnVar::from(ColorText)),
            (Color, CssFnVar::from(ColorBackground)),
            (FontSize, "0.8rem"),
            (FontStyle, "normal"),
            (FontWeight, "400"),
            (LineHeight, "1.25"),
            (TextAlign, "center"),
            (WhiteSpace, "normal"),
            (PointerEvents, "none"),
            (ZIndex, "2"),
        ]
    }
}

fn bubble_tail() -> CssRule {
    let declarations_block = declarations_block![
        (Content, r#""""#),
        (ZIndex, "-1"),
        (Position, "fixed"),
        (PositionAnchor, "--tip"),
        (PositionArea, "top"),
        (Width, "0.6rem"),
        (Height, "0.6rem"),
        (BackgroundColor, CssFnVar::from(ColorText)),
        (Transform, "rotate(45deg)"),
        (MarginBottom, "0.35rem"),
        (MarginTop, calc_anchor_size_block()),
        (PositionTryFallbacks, "flip-block"),
    ];

    CssRule::new()
        .selectors_list(".tip-bubble::before")
        .content(declarations_block)
}

fn tip_bubble_reveal() -> CssRule {
    CssRule::new()
        .push_selector(".tooltip:hover .tip-bubble")
        .push_selector(".tooltip:has(:focus-visible) .tip-bubble")
        .content(declarations_block![(Display, "block"), (Opacity, "1")])
}

fn tip_bubble_starting_style() -> Bake {
    let revealed = CssRule::new()
        .selectors_list(bake_comma![
            ".tooltip:hover .tip-bubble",
            ".tooltip:has(:focus-visible) .tip-bubble",
        ])
        .content(CssDeclaration::from(Opacity).content("0"));

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
