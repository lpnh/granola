//! Based on `button.css` from daisyUI by Pouya Saadeghi
//! Source: https://github.com/saadeghi/daisyui/blob/master/packages/daisyui/src/components/button.css
//! Licensed under MIT License (https://github.com/saadeghi/daisyui/blob/master/LICENSE)

use crate::{homemade::*, prelude::*, recipes::*};

const BTN_FG: &str = "btn-fg";

const BTN_BG: &str = "btn-bg";
const BTN_HOVER_BG: &str = "btn-hover-bg";

const BTN_BORDER: &str = "btn-border";

const BTN_ACTIVE_BG: &str = "btn-active-bg";
const BTN_ACTIVE_BORDER: &str = "btn-active-border";

const BTN_COLOR: &str = "btn-color";
const BTN_SIZE: &str = "btn-size";

/// The homemade recipe for the `btn` component.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let button = HtmlButton::from(Btn).content("x");
///
/// assert_eq!(button.bake(), r#"<button class="btn">x</button>"#);
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let stylesheet = CssStylesheet::from(Btn);
///
/// assert_eq!(
///     stylesheet.bake_pretty(),
///     r#".btn {
///   display: inline flex;
///   align-items: center;
///   justify-content: center;
///   padding: 0.6em 1.2em;
///   height: var(--btn-size);
///   font-size: 0.875rem;
///   font-weight: 500;
///   line-height: 1.25rem;
///   text-decoration: none;
///   white-space: nowrap;
///   border: 1px solid;
///   border-color: var(--btn-border);
///   border-radius: 0.5em;
///   background-color: var(--btn-bg);
///   color: var(--btn-fg);
///   cursor: pointer;
///   transition: background-color 150ms ease;
///   --btn-size: 2.5rem;
///   --btn-bg: var(--btn-color, var(--color-surface));
///   --btn-border: color-mix(in oklab, var(--btn-bg), #000 5%);
///   --btn-fg: var(--color-text);
/// }
/// .btn:hover {
///   --btn-bg: var(
///     --btn-hover-bg,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 7%)
///   );
/// }
/// .btn:active {
///   --btn-bg: var(
///     --btn-active-bg,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 5%)
///   );
///   --btn-border: var(
///     --btn-active-border,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 7%)
///   );
///   transform: scale(0.97);
/// }
/// .btn:focus-visible {
///   outline-width: 2px;
///   outline-style: solid;
///   outline-color: var(--btn-color, var(--color-text));
///   outline-offset: 2px;
/// }
/// .btn-primary {
///   --btn-color: var(--color-primary);
///   --btn-fg: var(--color-primary-text);
/// }
/// .btn-ghost {
///   color: var(--btn-color, var(--color-text));
///   --btn-bg: #0000;
///   --btn-border: #0000;
///   --btn-hover-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-text)) 10%,
///     #0000
///   );
///   --btn-active-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-text)) 20%,
///     #0000
///   );
///   --btn-active-border: #0000;
/// }
/// .btn-square {
///   padding: 0;
///   width: var(--btn-size);
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Btn;

impl ButtonRecipe for Btn {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("btn");
    }
}

impl ButtonRecipe for BtnPrimary {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("btn-primary");
    }
}

impl ButtonRecipe for BtnGhost {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("btn-ghost");
    }
}

impl ButtonRecipe for BtnSquare {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe(global_attrs: &mut GlobalAttrs) {
        global_attrs.class("btn-square");
    }
}

impl StylesheetRecipe for Btn {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([
            CssRule::from(Btn).into(),
            CssRule::from(BtnHover).into(),
            CssRule::from(BtnActive).into(),
            CssRule::from(BtnFocusVisible).into(),
            CssRule::from(BtnPrimary).into(),
            CssRule::from(BtnGhost).into(),
            CssRule::from(BtnSquare).into(),
        ])
    }
}

impl SimpleSelectorRecipe for Btn {
    fn selector_recipe(selector: &mut Bake) {
        *selector = ".btn".into();
    }
}

impl RuleRecipe for Btn {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(CssSimpleSelector::<Self>::from_cookbook());
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssDisplay::new().fold_in("inline").fold_in("flex").into(),
            CssAlignItems::from(Center).into(),
            CssJustifyContent::from(Center).into(),
            CssPadding::new().content("0.6em 1.2em").into(),
            CssHeight::new()
                .content(CssFnVar::new().custom_property(BTN_SIZE))
                .into(),
            CssFontSize::new().content("0.875rem").into(),
            CssFontWeight::new().content("500").into(),
            CssLineHeight::new().content("1.25rem").into(),
            CssTextDecoration::from(None).into(),
            CssWhiteSpace::from(Nowrap).into(),
            CssBorder::new().content("1px solid").into(),
            CssBorderColor::new()
                .content(CssFnVar::new().custom_property(BTN_BORDER))
                .into(),
            CssBorderRadius::new().content("0.5em").into(),
            CssBackgroundColor::new()
                .content(CssFnVar::new().custom_property(BTN_BG))
                .into(),
            CssColor::new()
                .content(CssFnVar::new().custom_property(BTN_FG))
                .into(),
            CssCursor::from(Pointer).into(),
            CssTransition::new()
                .content("background-color 150ms ease")
                .into(),
            CssCustomProperty::new()
                .name("btn-size")
                .value("2.5rem")
                .into(),
            CssCustomProperty::from(BtnBg)
                .value(btn_color_or_surface())
                .into(),
            CssCustomProperty::from(BtnBorder)
                .value(color_mix_darken(
                    CssFnVar::new().custom_property(BTN_BG),
                    "5%",
                ))
                .into(),
            CssCustomProperty::from(BtnFg)
                .value(CssFnVar::from(ColorText))
                .into(),
        ]);
    }
}

/// The homemade recipe for the `btn:hover` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let rule = CssRule::from(BtnHover);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     r#".btn:hover {
///   --btn-bg: var(
///     --btn-hover-bg,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 7%)
///   );
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnHover;

impl SimpleSelectorRecipe for BtnHover {
    fn selector_recipe(selector: &mut Bake) {
        *selector = ".btn:hover".into();
    }
}

impl RuleRecipe for BtnHover {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(CssSimpleSelector::<Self>::from_cookbook());
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.push_mut(
            CssCustomProperty::from(BtnBg).value(
                CssFnVar::new()
                    .custom_property(BTN_HOVER_BG)
                    .fallback(color_mix_darken(btn_color_or_surface(), "7%")),
            ),
        );
    }
}

/// The homemade recipe for the `btn:active` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let rule = CssRule::from(BtnActive);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     r#".btn:active {
///   --btn-bg: var(
///     --btn-active-bg,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 5%)
///   );
///   --btn-border: var(
///     --btn-active-border,
///     color-mix(in oklab, var(--btn-color, var(--color-surface)), #000 7%)
///   );
///   transform: scale(0.97);
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnActive;

impl SimpleSelectorRecipe for BtnActive {
    fn selector_recipe(selector: &mut Bake) {
        *selector = ".btn:active".into();
    }
}

impl RuleRecipe for BtnActive {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSimpleSelector::<Self>::from_cookbook().into();
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.declarations = vec![
            CssCustomProperty::from(BtnBg)
                .value(
                    CssFnVar::new()
                        .custom_property(BTN_ACTIVE_BG)
                        .fallback(color_mix_darken(btn_color_or_surface(), "5%")),
                )
                .into(),
            CssCustomProperty::from(BtnBorder)
                .value(
                    CssFnVar::new()
                        .custom_property(BTN_ACTIVE_BORDER)
                        .fallback(color_mix_darken(btn_color_or_surface(), "7%")),
                )
                .into(),
            CssTransform::new().content("scale(0.97)").into(),
        ];
    }
}

/// The homemade recipe for the `btn:focus-visible` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let rule = CssRule::from(BtnFocusVisible);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     r#".btn:focus-visible {
///   outline-width: 2px;
///   outline-style: solid;
///   outline-color: var(--btn-color, var(--color-text));
///   outline-offset: 2px;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnFocusVisible;

impl SimpleSelectorRecipe for BtnFocusVisible {
    fn selector_recipe(selector: &mut Bake) {
        *selector = ".btn:focus-visible".into();
    }
}

impl RuleRecipe for BtnFocusVisible {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSimpleSelector::<Self>::from_cookbook().into();
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.declarations = vec![
            CssOutlineWidth::new().content("2px").into(),
            CssOutlineStyle::from(Solid).into(),
            CssOutlineColor::new().content(btn_color_or_text()).into(),
            CssOutlineOffset::new().content("2px").into(),
        ];
    }
}

/// The homemade recipe for the `btn-primary` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let rule = CssRule::from(BtnPrimary);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     r#".btn-primary {
///   --btn-color: var(--color-primary);
///   --btn-fg: var(--color-primary-text);
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnPrimary;

impl RuleRecipe for BtnPrimary {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(CssSimpleSelector::<Self>::from_cookbook());
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssCustomProperty::new()
                .name(BTN_COLOR)
                .value(CssFnVar::from(ColorPrimary))
                .into(),
            CssCustomProperty::from(BtnFg)
                .value(CssFnVar::from(ColorPrimaryText))
                .into(),
        ]);
    }
}

impl SimpleSelectorRecipe for BtnPrimary {
    fn selector_recipe(selector: &mut Bake) {
        *selector = ".btn-primary".into();
    }
}

/// The homemade recipe for the `btn-ghost` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let rule = CssRule::from(BtnGhost);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     r#".btn-ghost {
///   color: var(--btn-color, var(--color-text));
///   --btn-bg: #0000;
///   --btn-border: #0000;
///   --btn-hover-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-text)) 10%,
///     #0000
///   );
///   --btn-active-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-text)) 20%,
///     #0000
///   );
///   --btn-active-border: #0000;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnGhost;

impl RuleRecipe for BtnGhost {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(CssSimpleSelector::<Self>::from_cookbook());
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssColor::new().content(btn_color_or_text()).into(),
            CssCustomProperty::from(BtnBg).value("#0000").into(),
            CssCustomProperty::from(BtnBorder).value("#0000").into(),
            CssCustomProperty::new()
                .name(BTN_HOVER_BG)
                .value(color_mix_fade("10%"))
                .into(),
            CssCustomProperty::new()
                .name("btn-active-bg")
                .value(color_mix_fade("20%"))
                .into(),
            CssCustomProperty::new()
                .name(BTN_ACTIVE_BORDER)
                .value("#0000")
                .into(),
        ]);
    }
}

impl SimpleSelectorRecipe for BtnGhost {
    fn selector_recipe(selector: &mut Bake) {
        *selector = ".btn-ghost".into();
    }
}

/// The homemade recipe for the `btn-square` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let rule = CssRule::from(BtnSquare);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     r#".btn-square {
///   padding: 0;
///   width: var(--btn-size);
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnSquare;

impl RuleRecipe for BtnSquare {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        selectors_list.push_mut(CssSimpleSelector::<Self>::from_cookbook());
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block
            .push_mut(CssPadding::new().content("0"))
            .push_mut(CssWidth::new().content(CssFnVar::new().custom_property(BTN_SIZE)));
    }
}

impl SimpleSelectorRecipe for BtnSquare {
    fn selector_recipe(selector: &mut Bake) {
        *selector = ".btn-square".into();
    }
}

/// The homemade recipe for the `btn-bg` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(BtnBg).value("#0000");
///
/// assert_eq!(custom_property.bake(), "--btn-bg: #0000;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnBg;

impl CustomPropertyRecipe for BtnBg {
    fn name_recipe(name: &mut Bake) {
        *name = BTN_BG.into();
    }
}

/// The homemade recipe for the `btn-fg` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(BtnFg).value("initial");
///
/// assert_eq!(custom_property.bake(), "--btn-fg: initial;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnFg;

impl CustomPropertyRecipe for BtnFg {
    fn name_recipe(name: &mut Bake) {
        *name = BTN_FG.into();
    }
}

/// The homemade recipe for the `btn-border` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(BtnBorder).value("initial");
///
/// assert_eq!(custom_property.bake(), "--btn-border: initial;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnBorder;

impl CustomPropertyRecipe for BtnBorder {
    fn name_recipe(name: &mut Bake) {
        *name = BTN_BORDER.into();
    }
}

fn color_mix_darken(base: CssFnVar, pct: &'static str) -> CssFnColorMix {
    CssFnColorMix::new()
        .interpolation(ColorSpace::Oklab)
        .color(base)
        .color_pct("#000", pct)
}

fn color_mix_fade(pct: &'static str) -> CssFnColorMix {
    CssFnColorMix::new()
        .interpolation(ColorSpace::Oklab)
        .color_pct(btn_color_or_text(), pct)
        .color("#0000")
}

fn btn_color_or_text() -> CssFnVar {
    CssFnVar::new()
        .custom_property(BTN_COLOR)
        .fallback(CssFnVar::from(CssCustomProperty::from(ColorText)))
}

fn btn_color_or_surface() -> CssFnVar {
    CssFnVar::new()
        .custom_property(BTN_COLOR)
        .fallback(CssFnVar::from(CssCustomProperty::from(ColorSurface)))
}
