use std::borrow::Cow;

use crate::{prelude::*, recipes::*};

const BTN_BG_DARKENED: &str =
    "color-mix(in oklab, var(--btn-color, var(--color-base-200)), #000 7%)";

/// The homemade recipe for the `btn` stylesheet.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let rule = CssRule::from(Btn);
///
/// assert_eq!(
///     rule.bake(),
///     ".btn {
///   display: inline flex;
///   align-items: center;
///   justify-content: center;
///   padding: 0.6em 1.2em;
///   font-size: 0.875rem;
///   font-weight: 500;
///   line-height: 1.25rem;
///   text-decoration: none;
///   white-space: nowrap;
///   border: none;
///   border-radius: 0.4em;
///   background-color: var(--btn-bg);
///   color: var(--btn-fg);
///   cursor: pointer;
///   font-family: inherit;
///   transition: background-color 150ms ease;
///   --btn-bg: var(--btn-color, var(--color-base-200));
///   --btn-fg: var(--color-base-content);
/// }"
/// );
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let stylesheet: CssStylesheet<Btn> = CssStylesheet::from_cookbook();
///
/// assert_eq!(
///     stylesheet.bake(),
///     ".btn {
///   display: inline flex;
///   align-items: center;
///   justify-content: center;
///   padding: 0.6em 1.2em;
///   font-size: 0.875rem;
///   font-weight: 500;
///   line-height: 1.25rem;
///   text-decoration: none;
///   white-space: nowrap;
///   border: none;
///   border-radius: 0.4em;
///   background-color: var(--btn-bg);
///   color: var(--btn-fg);
///   cursor: pointer;
///   font-family: inherit;
///   transition: background-color 150ms ease;
///   --btn-bg: var(--btn-color, var(--color-base-200));
///   --btn-fg: var(--color-base-content);
/// }
///
/// .btn:hover {
///   --btn-bg: color-mix(in oklab, var(--btn-color, var(--color-base-200)), #000 7%);
/// }
///
/// .btn:active {
///   --btn-bg: color-mix(in oklab, var(--btn-color, var(--color-base-200)), #000 7%);
///   transform: scale(0.97);
/// }
///
/// .btn:focus-visible {
///   outline-width: 2px;
///   outline-style: solid;
///   outline-color: var(--btn-color, var(--color-base-200));
///   outline-offset: 2px;
/// }
///
/// .btn-primary {
///   --btn-color: var(--color-primary);
///   --btn-fg: var(--color-primary-content);
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Btn;

impl StylesheetRecipe for Btn {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([
            CssRule::from(Btn).into(),
            CssRule::from(BtnHover).into(),
            CssRule::from(BtnActive).into(),
            CssRule::from(BtnFocusVisible).into(),
            CssRule::from(BtnPrimary).into(),
        ])
    }
}

impl SimpleSelectorRecipe for Btn {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = ".btn".into();
    }
}

impl RuleRecipe for Btn {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSimpleSelector::<Self>::from_cookbook().into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            CssDisplay::from((Inline, Flex)).into(),
            CssAlignItems::from(Center).into(),
            CssJustifyContent::<Center>::from_cookbook().into(),
            CssPadding::new().content("0.6em 1.2em").into(),
            CssFontSize::new().content("0.875rem").into(),
            CssFontWeight::new().content("500").into(),
            CssLineHeight::new().content("1.25rem").into(),
            CssTextDecoration::from(None).into(),
            CssWhiteSpace::<Nowrap>::from_cookbook().into(),
            CssBorder::from(None).into(),
            CssBorderRadius::new().content("0.4em").into(),
            CssBackgroundColor::new().content("var(--btn-bg)").into(),
            CssColor::new().content("var(--btn-fg)").into(),
            CssCursor::<Pointer>::from_cookbook().into(),
            CssFontFamily::from(Inherit).into(),
            CssTransition::new()
                .content("background-color 150ms ease")
                .into(),
            ("--btn-bg", "var(--btn-color, var(--color-base-200))").into(),
            ("--btn-fg", "var(--color-base-content)").into(),
        ];
    }
}

/// The homemade recipe for the `btn:hover` rule.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnHover;

impl SimpleSelectorRecipe for BtnHover {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = ".btn:hover".into();
    }
}

impl RuleRecipe for BtnHover {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSimpleSelector::<Self>::from_cookbook().into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![("--btn-bg", BTN_BG_DARKENED).into()];
    }
}

/// The homemade recipe for the `btn:active` rule.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnActive;

impl SimpleSelectorRecipe for BtnActive {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = ".btn:active".into();
    }
}

impl RuleRecipe for BtnActive {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSimpleSelector::<Self>::from_cookbook().into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            ("--btn-bg", BTN_BG_DARKENED).into(),
            CssTransform::new().content("scale(0.97)").into(),
        ];
    }
}

/// The homemade recipe for the `btn:focus-visible` rule.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnFocusVisible;

impl SimpleSelectorRecipe for BtnFocusVisible {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = ".btn:focus-visible".into();
    }
}

impl RuleRecipe for BtnFocusVisible {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSimpleSelector::<Self>::from_cookbook().into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            CssOutlineWidth::new().content("2px").into(),
            CssOutlineStyle::<Solid>::from_cookbook().into(),
            CssOutlineColor::new()
                .content("var(--btn-color, var(--color-base-200))")
                .into(),
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
/// let rule: CssRule<BtnPrimary> = CssRule::from_cookbook();
///
/// assert_eq!(
///     rule.bake(),
///     ".btn-primary {
///   --btn-color: var(--color-primary);
///   --btn-fg: var(--color-primary-content);
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnPrimary;

impl RuleRecipe for BtnPrimary {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSimpleSelector::<Self>::from_cookbook().into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            ("--btn-color", "var(--color-primary)").into(),
            ("--btn-fg", "var(--color-primary-content)").into(),
        ];
    }
}

impl SimpleSelectorRecipe for BtnPrimary {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = ".btn-primary".into();
    }
}
