use std::borrow::Cow;

use crate::{homemade::*, prelude::*, recipes::*};

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
///     rule.bake_pretty(),
///     r#".btn {
///   display: inline flex;
///   align-items: center;
///   justify-content: center;
///   padding: 0.6em 1.2em;
///   font-size: 0.875rem;
///   font-weight: 500;
///   line-height: 1.25rem;
///   text-decoration: none;
///   white-space: nowrap;
///   border: 1px solid;
///   border-color: var(--btn-border);
///   border-radius: 0.4em;
///   background-color: var(--btn-bg);
///   color: var(--btn-fg);
///   cursor: pointer;
///   transition: background-color 150ms ease;
///   --btn-bg: var(--btn-color, var(--color-surface));
///   --btn-border: color-mix(in oklab, var(--btn-bg), #000 5%);
///   --btn-fg: var(--color-text);
/// }
/// "#
/// );
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
///   font-size: 0.875rem;
///   font-weight: 500;
///   line-height: 1.25rem;
///   text-decoration: none;
///   white-space: nowrap;
///   border: 1px solid;
///   border-color: var(--btn-border);
///   border-radius: 0.4em;
///   background-color: var(--btn-bg);
///   color: var(--btn-fg);
///   cursor: pointer;
///   transition: background-color 150ms ease;
///   --btn-bg: var(--btn-color, var(--color-surface));
///   --btn-border: color-mix(in oklab, var(--btn-bg), #000 5%);
///   --btn-fg: var(--color-text);
/// }
/// .btn:hover {
///   --btn-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-surface)),
///     #000 7%
///   );
/// }
/// .btn:active {
///   --btn-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-surface)),
///     #000 5%
///   );
///   --btn-border: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-surface)),
///     #000 7%
///   );
///   transform: scale(0.97);
/// }
/// .btn:focus-visible {
///   outline-width: 2px;
///   outline-style: solid;
///   outline-color: var(--btn-color, var(--color-surface));
///   outline-offset: 2px;
/// }
/// .btn-primary {
///   --btn-color: var(--color-primary);
///   --btn-fg: var(--color-text);
/// }
/// "#
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
        selectors_list.push_mut(CssSimpleSelector::<Self>::from_cookbook());
    }

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.extend_mut([
            CssDisplay::from((Inline, Flex)).into(),
            CssAlignItems::from(Center).into(),
            CssJustifyContent::from(Center).into(),
            CssPadding::new().content("0.6em 1.2em").into(),
            CssFontSize::new().content("0.875rem").into(),
            CssFontWeight::new().content("500").into(),
            CssLineHeight::new().content("1.25rem").into(),
            CssTextDecoration::from(None).into(),
            CssWhiteSpace::from(Nowrap).into(),
            CssBorder::new().content("1px solid").into(),
            CssBorderColor::new()
                .content(CssFnVar::from(BtnBorder))
                .into(),
            CssBorderRadius::new().content("0.4em").into(),
            CssBackgroundColor::new()
                .content(CssFnVar::from(BtnBg))
                .into(),
            CssColor::new().content(CssFnVar::from(BtnFg)).into(),
            CssCursor::from(Pointer).into(),
            CssTransition::new()
                .content("background-color 150ms ease")
                .into(),
            CssCustomProperty::from(BtnBg)
                .value(CssFnVar::from(BtnColor))
                .into(),
            CssCustomProperty::from(BtnBorder)
                .value(CssFnColorMix::new().content(bake![
                    "in oklab, ",
                    CssFnVar::from(BtnBg),
                    ", #000 5%"
                ]))
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
///   --btn-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-surface)),
///     #000 7%
///   );
/// }
/// "#
/// );
/// ```
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

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.declarations = vec![
            CssCustomProperty::from(BtnBg)
                .value(CssFnColorMix::new().content(bake![
                    "in oklab, ",
                    CssFnVar::from(BtnColor),
                    ", #000 7%"
                ]))
                .into(),
        ];
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
///   --btn-bg: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-surface)),
///     #000 5%
///   );
///   --btn-border: color-mix(
///     in oklab,
///     var(--btn-color, var(--color-surface)),
///     #000 7%
///   );
///   transform: scale(0.97);
/// }
/// "#
/// );
/// ```
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

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.declarations = vec![
            CssCustomProperty::from(BtnBg)
                .value(CssFnColorMix::new().content(bake![
                    "in oklab, ",
                    CssFnVar::from(BtnColor),
                    ", #000 5%"
                ]))
                .into(),
            CssCustomProperty::from(BtnBorder)
                .value(CssFnColorMix::new().content(bake![
                    "in oklab, ",
                    CssFnVar::from(BtnColor),
                    ", #000 7%"
                ]))
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
///   outline-color: var(--btn-color, var(--color-surface));
///   outline-offset: 2px;
/// }
/// "#
/// );
/// ```
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

    fn declarations_block_recipe(declarations_block: &mut CssDeclarationsBlock) {
        declarations_block.declarations = vec![
            CssOutlineWidth::new().content("2px").into(),
            CssOutlineStyle::from(Solid).into(),
            CssOutlineColor::new()
                .content(CssFnVar::from(BtnColor))
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
/// let rule = CssRule::from(BtnPrimary);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     r#".btn-primary {
///   --btn-color: var(--color-primary);
///   --btn-fg: var(--color-text);
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
            CssCustomProperty::from(BtnColor)
                .value(CssFnVar::from(ColorPrimary))
                .into(),
            CssCustomProperty::from(BtnFg)
                .value(CssFnVar::from(ColorText))
                .into(),
        ]);
    }
}

impl SimpleSelectorRecipe for BtnPrimary {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = ".btn-primary".into();
    }
}

/// The homemade recipe for the `btn-color` custom property.
///
/// As a `var()` reference it falls back to the surface color.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(BtnColor).value(CssFnVar::from(ColorPrimary));
///
/// assert_eq!(custom_property.bake(), "--btn-color: var(--color-primary);");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(BtnColor);
///
/// assert_eq!(var_fn.bake(), "var(--btn-color, var(--color-surface))");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnColor;

impl CustomPropertyRecipe for BtnColor {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "btn-color".into();
    }
}

impl FnVarRecipe for BtnColor {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = bake_block!["--btn-color,", CssFnVar::from(ColorSurface)].into();
    }
}

/// The homemade recipe for the `btn-bg` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(BtnBg).value(CssFnVar::from(BtnColor));
///
/// assert_eq!(
///     custom_property.bake(),
///     "--btn-bg: var(--btn-color, var(--color-surface));"
/// );
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(BtnBg);
///
/// assert_eq!(var_fn.bake(), "var(--btn-bg)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnBg;

impl CustomPropertyRecipe for BtnBg {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "btn-bg".into();
    }
}

impl FnVarRecipe for BtnBg {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "--btn-bg".into();
    }
}

/// The homemade recipe for the `btn-fg` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(BtnFg).value(CssFnVar::from(ColorText));
///
/// assert_eq!(custom_property.bake(), "--btn-fg: var(--color-text);");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(BtnFg);
///
/// assert_eq!(var_fn.bake(), "var(--btn-fg)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnFg;

impl CustomPropertyRecipe for BtnFg {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "btn-fg".into();
    }
}

impl FnVarRecipe for BtnFg {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "--btn-fg".into();
    }
}

/// The homemade recipe for the `btn-border` custom property.
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let custom_property = CssCustomProperty::from(BtnBorder).value(CssFnVar::from(BtnBg));
///
/// assert_eq!(custom_property.bake(), "--btn-border: var(--btn-bg);");
/// ```
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let var_fn = CssFnVar::from(BtnBorder);
///
/// assert_eq!(var_fn.bake(), "var(--btn-border)");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BtnBorder;

impl CustomPropertyRecipe for BtnBorder {
    fn name_recipe(name: &mut Cow<'static, str>) {
        *name = "btn-border".into();
    }
}

impl FnVarRecipe for BtnBorder {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "--btn-border".into();
    }
}
