//! Based on "A (more) Modern CSS Reset" by Andy Bell
//! Source: https://piccalil.li/blog/a-more-modern-css-reset/
//! Licensed under CC BY 3.0 (https://creativecommons.org/licenses/by/3.0/)

use crate::{prelude::*, recipes::*};

/// The "(more) Modern CSS Reset" stylesheet recipe.
///
/// [Andy Bell's post with source code](https://piccalil.li/blog/a-more-modern-css-reset/)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let stylesheet = CssStylesheet::from(AndyBell);
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
pub struct AndyBell;

impl StylesheetRecipe for AndyBell {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([
            CssRule::from(BoxSizingReset).into(),
            text_size_adjust_reset(),
            default_margin_reset(),
            list_style_reset(),
            body_defaults(),
            headings_and_forms_line_height(),
            headings_text_wrap(),
            unclassed_anchor_default_style(),
            images_width_and_display(),
            form_controls_font_inherit(),
            textarea_min_height(),
            target_scroll_margin(),
        ]);
    }
}

fn text_size_adjust_reset() -> CssStatement {
    let selectors_list = "html";
    let declarations_block: [CssDeclaration; 3] = [
        ("-moz-text-size-adjust", "none").into(),
        CssWebkitTextSizeAdjust::from(None).into(),
        CssTextSizeAdjust::from(None).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn default_margin_reset() -> CssStatement {
    let selectors_list = [
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
    ];
    let declarations_block = CssMarginBlockEnd::new().content("0");

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn list_style_reset() -> CssStatement {
    let selectors_list = ["ul[role='list']", "ol[role='list']"];
    let declarations_block = CssListStyle::from(None);

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn body_defaults() -> CssStatement {
    let selectors_list = "body";
    let declarations_block: [CssDeclaration; 2] = [
        CssMinHeight::new().content("100vh").into(),
        CssLineHeight::new().content("1.5").into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn headings_and_forms_line_height() -> CssStatement {
    let selectors_list = CssSelectorsList::from(Headings)
        .push("button")
        .push("input")
        .push("label")
        .bake_recipe();
    let declarations_block = CssLineHeight::new().content("1.1");

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn headings_text_wrap() -> CssStatement {
    let selectors_list = CssSelectorsList::from(Headings).bake_recipe();
    let declarations_block = CssTextWrap::from(Balance);

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn unclassed_anchor_default_style() -> CssStatement {
    let selectors_list = "a:not([class])";
    let declarations_block: [CssDeclaration; 2] = [
        CssTextDecorationSkipInk::from(Auto).into(),
        CssColor::from(Currentcolor).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn images_width_and_display() -> CssStatement {
    let selectors_list = ["img", "picture"];
    let declarations_block: [CssDeclaration; 2] = [
        CssMaxWidth::new().content("100%").into(),
        CssDisplay::from(Block).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn form_controls_font_inherit() -> CssStatement {
    let selectors_list = CssSelectorsList::from(FormControls).bake_recipe();
    let declarations_block: [CssDeclaration; 2] = [
        CssFontFamily::from(Inherit).into(),
        CssFontSize::from(Inherit).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn textarea_min_height() -> CssStatement {
    CssRule::new()
        .selectors_list("textarea:not([rows])")
        .declarations_block(CssMinHeight::new().content("10em"))
        .into()
}

fn target_scroll_margin() -> CssStatement {
    CssRule::new()
        .selectors_list(":target")
        .declarations_block(CssScrollMarginBlock::new().content("5ex"))
        .into()
}
