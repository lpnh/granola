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
/// let rule: CssRule<Btn> = CssRule::from_recipe();
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
/// let stylesheet: CssStylesheet<Btn> = CssStylesheet::from_recipe();
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
        statements.push(CssRule::<Btn>::from_recipe().into());
        statements.push(CssRule::<BtnHover>::from_recipe().into());
        statements.push(CssRule::<BtnActive>::from_recipe().into());
        statements.push(CssRule::<BtnFocusVisible>::from_recipe().into());
        statements.push(CssRule::<BtnPrimary>::from_recipe().into());
    }
}

impl SimpleSelectorRecipe for Btn {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = ".btn".into();
    }
}

impl RuleRecipe for Btn {
    fn selectors_list_recipe(selectors_list: &mut CssSelectorsList) {
        *selectors_list = CssSimpleSelector::<Self>::from_recipe().into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            CssDisplay::<(Inline, Flex)>::from_recipe().into(),
            CssAlignItems::<Center>::from_recipe().into(),
            CssJustifyContent::<Center>::from_recipe().into(),
            CssPadding::<()>::new("0.6em 1.2em").into(),
            CssFontSize::<()>::new("0.875rem").into(),
            CssFontWeight::<()>::new("500").into(),
            CssLineHeight::<()>::new("1.25rem").into(),
            CssTextDecoration::<None>::from_recipe().into(),
            CssWhiteSpace::<Nowrap>::from_recipe().into(),
            CssBorder::<None>::from_recipe().into(),
            CssBorderRadius::<()>::new("0.4em").into(),
            CssBackgroundColor::<()>::new("var(--btn-bg)").into(),
            CssColor::<()>::new("var(--btn-fg)").into(),
            CssCursor::<Pointer>::from_recipe().into(),
            CssFontFamily::<Inherit>::from_recipe().into(),
            CssTransition::<()>::new("background-color 150ms ease").into(),
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
        *selectors_list = CssSimpleSelector::<Self>::from_recipe().into();
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
        *selectors_list = CssSimpleSelector::<Self>::from_recipe().into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            ("--btn-bg", BTN_BG_DARKENED).into(),
            CssTransform::<()>::new("scale(0.97)").into(),
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
        *selectors_list = CssSimpleSelector::<Self>::from_recipe().into();
    }

    fn declarations_block_recipe(properties_list: &mut CssDeclarationsBlock) {
        properties_list.declarations = vec![
            CssOutlineWidth::<()>::new("2px").into(),
            CssOutlineStyle::<Solid>::from_recipe().into(),
            CssOutlineColor::<()>::new("var(--btn-color, var(--color-base-200))").into(),
            CssOutlineOffset::<()>::new("2px").into(),
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
/// let rule: CssRule<BtnPrimary> = CssRule::from_recipe();
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
        *selectors_list = CssSimpleSelector::<Self>::from_recipe().into();
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
