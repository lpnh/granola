// Based on "A (more) Modern CSS Reset" by Andy Bell
// Source: https://piccalil.li/blog/a-more-modern-css-reset/
// Licensed under CC BY 3.0 (https://creativecommons.org/licenses/by/3.0/)

use crate::{cookbook::*, prelude::*};

/// The "(more) Modern CSS Reset" stylesheet recipe.
///
/// [Andy Bell's post with source code](https://piccalil.li/blog/a-more-modern-css-reset/)
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let stylesheet: CssStylesheet<ModernCSSReset> = CssStylesheet::from_recipe();
///
/// assert_eq!(
///     stylesheet.bake(),
///     "*,
/// ::after,
/// ::before {
///   box-sizing: border-box;
/// }
///
/// html {
///   -moz-text-size-adjust: none;
///   -webkit-text-size-adjust: none;
///   text-size-adjust: none;
/// }
///
/// body,
/// h1,
/// h2,
/// h3,
/// h4,
/// p,
/// figure,
/// blockquote,
/// dl,
/// dd {
///   margin-block-end: 0;
/// }
///
/// ul[role='list'],
/// ol[role='list'] {
///   list-style: none;
/// }
///
/// body {
///   min-height: 100vh;
///   line-height: 1.5;
/// }
///
/// h1,
/// h2,
/// h3,
/// h4,
/// button,
/// input,
/// label {
///   line-height: 1.1;
/// }
///
/// h1,
/// h2,
/// h3,
/// h4 {
///   text-wrap: balance;
/// }
///
/// a:not([class]) {
///   text-decoration-skip-ink: auto;
///   color: currentcolor;
/// }
///
/// img,
/// picture {
///   max-width: 100%;
///   display: block;
/// }
///
/// button,
/// input,
/// select,
/// textarea {
///   font-family: inherit;
///   font-size: inherit;
/// }
///
/// textarea:not([rows]) {
///   min-height: 10em;
/// }
///
/// :target {
///   scroll-margin-block: 5ex;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModernCSSReset;

impl StylesheetRecipe for ModernCSSReset {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.push(CssRule::<BoxSizingReset>::from_recipe().into());
        statements.push(text_size_adjust_reset());
        statements.push(default_margin_reset());
        statements.push(list_style_reset());
        statements.push(body_defaults());
        statements.push(headings_and_forms_line_height());
        statements.push(headings_text_wrap());
        statements.push(unclassed_anchor_default_style());
        statements.push(images_width_and_display());
        statements.push(form_controls_font_inherit());
        statements.push(textarea_min_height());
        statements.push(target_scroll_margin());
    }
}

fn text_size_adjust_reset() -> CssStatement {
    let selectors_list: CssSelectorsList = "html".into();
    let declarations_block: [CssDeclaration; 3] = [
        ("-moz-text-size-adjust", "none").into(),
        CssWebkitTextSizeAdjust::<None>::from_recipe().into(),
        CssTextSizeAdjust::<None>::from_recipe().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn default_margin_reset() -> CssStatement {
    let selectors_list: CssSelectorsList = [
        "body",
        "h1",
        "h2",
        "h3",
        "h4",
        "p",
        "figure",
        "blockquote",
        "dl",
        "dd",
    ]
    .into();
    let declarations_block = ("margin-block-end", "0");

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn list_style_reset() -> CssStatement {
    let selectors_list: CssSelectorsList = ["ul[role='list']", "ol[role='list']"].into();
    let declarations_block = CssListStyle::<None>::from_recipe();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn body_defaults() -> CssStatement {
    let selectors_list: CssSelectorsList = "body".into();
    let declarations_block: [CssDeclaration; 2] = [
        ("min-height", "100vh").into(),
        CssLineHeight::<()>::new("1.5").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn headings_and_forms_line_height() -> CssStatement {
    let selectors_list = CssSelectorsList::<Headings>::from_recipe()
        .push("button")
        .push("input")
        .push("label")
        .bake_recipe();
    let declarations_block = CssLineHeight::<()>::new("1.1");

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn headings_text_wrap() -> CssStatement {
    let selectors_list = CssSelectorsList::<Headings>::from_recipe().bake_recipe();
    let declarations_block = CssTextWrap::<Balance>::from_recipe();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn unclassed_anchor_default_style() -> CssStatement {
    let selectors_list: CssSelectorsList = "a:not([class])".into();
    let declarations_block: [CssDeclaration; 2] = [
        CssTextDecorationSkipInk::<Auto>::from_recipe().into(),
        CssColor::<Currentcolor>::from_recipe().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn images_width_and_display() -> CssStatement {
    let selectors_list: CssSelectorsList = ["img", "picture"].into();
    let declarations_block: [CssDeclaration; 2] = [
        ("max-width", "100%").into(),
        CssDisplay::<Block>::from_recipe().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn form_controls_font_inherit() -> CssStatement {
    let selectors_list = CssSelectorsList::<FormControls>::from_recipe().bake_recipe();
    let declarations_block: [CssDeclaration; 2] = [
        CssFontFamily::<Inherit>::from_recipe().into(),
        CssFontSize::<Inherit>::from_recipe().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn textarea_min_height() -> CssStatement {
    CssRule::<()>::new("textarea:not([rows])", ("min-height", "10em")).into()
}

fn target_scroll_margin() -> CssStatement {
    CssRule::<()>::new(":target", ("scroll-margin-block", "5ex")).into()
}
