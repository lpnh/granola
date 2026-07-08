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

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("btn")
    }
}

impl ButtonRecipe for BtnPrimary {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("btn-primary")
    }
}

impl ButtonRecipe for BtnGhost {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("btn-ghost")
    }
}

impl ButtonRecipe for BtnSquare {
    recipe_boilerplate!(ButtonRecipe);

    fn global_attrs_recipe() -> GlobalAttrs {
        GlobalAttrs::default().class("btn-square")
    }
}

impl StylesheetRecipe for Btn {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(Btn),
            CssRule::from(BtnHover),
            CssRule::from(BtnActive),
            CssRule::from(BtnFocusVisible),
            CssRule::from(BtnPrimary),
            CssRule::from(BtnGhost),
            CssRule::from(BtnSquare),
        ]
    }
}

impl SimpleSelectorRecipe for Btn {
    fn selector_recipe() -> Bake {
        ".btn".into()
    }
}

impl RuleRecipe for Btn {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        CssSimpleSelector::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(Display).content("inline flex"),
            CssDeclaration::from(AlignItems).content("center"),
            CssDeclaration::from(JustifyContent).content("center"),
            CssDeclaration::from(Padding).content("0.6em 1.2em"),
            CssDeclaration::from(Height).content(CssFnVar::new().custom_property(BTN_SIZE)),
            CssDeclaration::from(FontSize).content("0.875rem"),
            CssDeclaration::from(FontWeight).content("500"),
            CssDeclaration::from(LineHeight).content("1.25rem"),
            CssDeclaration::from(TextDecoration).content("none"),
            CssDeclaration::from(WhiteSpace).content("nowrap"),
            CssDeclaration::from(Border).content("1px solid"),
            CssDeclaration::from(BorderColor).content(CssFnVar::new().custom_property(BTN_BORDER)),
            CssDeclaration::from(BorderRadius).content("0.5em"),
            CssDeclaration::from(BackgroundColor).content(CssFnVar::new().custom_property(BTN_BG)),
            CssDeclaration::from(Color).content(CssFnVar::new().custom_property(BTN_FG)),
            CssDeclaration::from(Cursor).content("pointer"),
            CssDeclaration::from(Transition).content("background-color 150ms ease"),
            CssDeclaration::from(BtnSize).content("2.5rem"),
            CssDeclaration::from(BtnBg).content(btn_color_or_surface()),
            CssDeclaration::from(BtnBorder).content(color_mix_darken(
                CssFnVar::new().custom_property(BTN_BG),
                "5%"
            )),
            CssDeclaration::from(BtnFg).content(CssFnVar::from(ColorText)),
        ]
    }
}

/// The homemade recipe for the `btn:hover` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
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
    fn selector_recipe() -> Bake {
        ".btn:hover".into()
    }
}

impl RuleRecipe for BtnHover {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        CssSimpleSelector::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from(BtnBg)
            .content(
                CssFnVar::new()
                    .custom_property(BTN_HOVER_BG)
                    .fallback(color_mix_darken(btn_color_or_surface(), "7%")),
            )
            .into()
    }
}

/// The homemade recipe for the `btn:active` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
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
    fn selector_recipe() -> Bake {
        ".btn:active".into()
    }
}

impl RuleRecipe for BtnActive {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        CssSimpleSelector::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(BtnBg).content(
                CssFnVar::new()
                    .custom_property(BTN_ACTIVE_BG)
                    .fallback(color_mix_darken(btn_color_or_surface(), "5%")),
            ),
            CssDeclaration::from(BtnBorder).content(
                CssFnVar::new()
                    .custom_property(BTN_ACTIVE_BORDER)
                    .fallback(color_mix_darken(btn_color_or_surface(), "7%")),
            ),
            CssDeclaration::from(Transform).content("scale(0.97)"),
        ]
    }
}

/// The homemade recipe for the `btn:focus-visible` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
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
    fn selector_recipe() -> Bake {
        ".btn:focus-visible".into()
    }
}

impl RuleRecipe for BtnFocusVisible {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        CssSimpleSelector::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(OutlineWidth).content("2px"),
            CssDeclaration::from(OutlineStyle).content("solid"),
            CssDeclaration::from(OutlineColor).content(btn_color_or_text()),
            CssDeclaration::from(OutlineOffset).content("2px"),
        ]
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
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        CssSimpleSelector::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(BtnColor).content(CssFnVar::from(ColorPrimary)),
            CssDeclaration::from(BtnFg).content(CssFnVar::from(ColorPrimaryText)),
        ]
    }
}

impl SimpleSelectorRecipe for BtnPrimary {
    fn selector_recipe() -> Bake {
        ".btn-primary".into()
    }
}

/// The homemade recipe for the `btn-ghost` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
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
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        CssSimpleSelector::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(Color).content(btn_color_or_text()),
            CssDeclaration::from(BtnBg).content("#0000"),
            CssDeclaration::from(BtnBorder).content("#0000"),
            CssDeclaration::from(BtnHoverBg).content(color_mix_fade("10%")),
            CssDeclaration::from(BtnActiveBg).content(color_mix_fade("20%")),
            CssDeclaration::from(BtnActiveBorder).content("#0000"),
        ]
    }
}

impl SimpleSelectorRecipe for BtnGhost {
    fn selector_recipe() -> Bake {
        ".btn-ghost".into()
    }
}

/// The homemade recipe for the `btn-square` rule.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
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
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        CssSimpleSelector::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(Padding).content("0"),
            CssDeclaration::from(Width).content(CssFnVar::new().custom_property(BTN_SIZE)),
        ]
    }
}

impl SimpleSelectorRecipe for BtnSquare {
    fn selector_recipe() -> Bake {
        ".btn-square".into()
    }
}

/// The homemade recipe for the `btn-bg` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(BtnBg).content("#0000");
///
/// assert_eq!(custom_property.bake(), "--btn-bg: #0000;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnBg;

impl CustomPropertyRecipe for BtnBg {
    fn name_recipe() -> Bake {
        BTN_BG.into()
    }
}

impl DeclarationRecipe for BtnBg {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }
}

/// The homemade recipe for the `btn-fg` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(BtnFg).initial();
///
/// assert_eq!(custom_property.bake(), "--btn-fg: initial;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnFg;

impl CustomPropertyRecipe for BtnFg {
    fn name_recipe() -> Bake {
        BTN_FG.into()
    }
}

impl DeclarationRecipe for BtnFg {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }
}

/// The homemade recipe for the `btn-border` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(BtnBorder).initial();
///
/// assert_eq!(custom_property.bake(), "--btn-border: initial;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnBorder;

impl CustomPropertyRecipe for BtnBorder {
    fn name_recipe() -> Bake {
        BTN_BORDER.into()
    }
}

impl DeclarationRecipe for BtnBorder {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }
}

/// The homemade recipe for the `btn-size` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(BtnSize).content("2.5rem");
///
/// assert_eq!(custom_property.bake(), "--btn-size: 2.5rem;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnSize;

impl CustomPropertyRecipe for BtnSize {
    fn name_recipe() -> Bake {
        BTN_SIZE.into()
    }
}

impl DeclarationRecipe for BtnSize {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }
}

/// The homemade recipe for the `btn-color` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(BtnColor).initial();
///
/// assert_eq!(custom_property.bake(), "--btn-color: initial;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnColor;

impl CustomPropertyRecipe for BtnColor {
    fn name_recipe() -> Bake {
        BTN_COLOR.into()
    }
}

impl DeclarationRecipe for BtnColor {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }
}

/// The homemade recipe for the `btn-hover-bg` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(BtnHoverBg).initial();
///
/// assert_eq!(custom_property.bake(), "--btn-hover-bg: initial;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnHoverBg;

impl CustomPropertyRecipe for BtnHoverBg {
    fn name_recipe() -> Bake {
        BTN_HOVER_BG.into()
    }
}

impl DeclarationRecipe for BtnHoverBg {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }
}

/// The homemade recipe for the `btn-active-bg` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(BtnActiveBg).initial();
///
/// assert_eq!(custom_property.bake(), "--btn-active-bg: initial;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnActiveBg;

impl CustomPropertyRecipe for BtnActiveBg {
    fn name_recipe() -> Bake {
        BTN_ACTIVE_BG.into()
    }
}

impl DeclarationRecipe for BtnActiveBg {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }
}

/// The homemade recipe for the `btn-active-border` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(BtnActiveBorder).initial();
///
/// assert_eq!(custom_property.bake(), "--btn-active-border: initial;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnActiveBorder;

impl CustomPropertyRecipe for BtnActiveBorder {
    fn name_recipe() -> Bake {
        BTN_ACTIVE_BORDER.into()
    }
}

impl DeclarationRecipe for BtnActiveBorder {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
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
