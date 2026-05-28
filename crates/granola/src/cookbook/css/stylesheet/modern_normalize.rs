// Based on modern-normalize v3.0.1 by Sindre Sorhus
// Source: https://github.com/sindresorhus/modern-normalize
// Licensed under MIT License

use crate::{cookbook::*, prelude::*};

/// The modern-normalize stylesheet recipe.
///
/// [modern-normalize source code](https://github.com/sindresorhus/modern-normalize/blob/main/modern-normalize.css)
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let stylesheet: CssStylesheet<ModernNormalize> = CssStylesheet::from_recipe();
///
/// assert_eq!(
///     stylesheet.bake(),
///     r#"*, *::before, *::after {
///   box-sizing: border-box;
/// }
///
/// html {
///   font-family: system-ui, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji';
///   line-height: 1.15;
///   -webkit-text-size-adjust: 100%;
///   tab-size: 4;
/// }
///
/// body {
///   margin: 0;
/// }
///
/// b, strong {
///   font-weight: bolder;
/// }
///
/// code, kbd, samp, pre {
///   font-family: ui-monospace, SFMono-Regular, Consolas, 'Liberation Mono', Menlo, monospace;
///   font-size: 1em;
/// }
///
/// small {
///   font-size: 80%;
/// }
///
/// sub, sup {
///   font-size: 75%;
///   line-height: 0;
///   position: relative;
///   vertical-align: baseline;
/// }
///
/// sub {
///   bottom: -0.25em;
/// }
///
/// sup {
///   top: -0.5em;
/// }
///
/// table {
///   border-color: currentcolor;
/// }
///
/// button, input, optgroup, select, textarea {
///   font-family: inherit;
///   font-size: 100%;
///   line-height: 1.15;
///   margin: 0;
/// }
///
/// button, [type="button"], [type="reset"], [type="submit"] {
///   -webkit-appearance: button;
/// }
///
/// legend {
///   padding: 0;
/// }
///
/// progress {
///   vertical-align: baseline;
/// }
///
/// ::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
///   height: auto;
/// }
///
/// [type="search"] {
///   -webkit-appearance: textfield;
///   outline-offset: -2px;
/// }
///
/// ::-webkit-search-decoration {
///   -webkit-appearance: none;
/// }
///
/// ::-webkit-file-upload-button {
///   -webkit-appearance: button;
///   font: inherit;
/// }
///
/// summary {
///   display: list-item;
/// }"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModernNormalize;

impl StylesheetRecipe for ModernNormalize {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.push(CssRule::<BoxSizingReset>::from_recipe().into());
        statements.push(html_defaults());
        statements.push(body_defaults());
        statements.push(b_strong_font_weight());
        statements.push(monospace_defaults());
        statements.push(small_font_size());
        statements.push(sub_sup_defaults());
        statements.push(sub_vertical_pos());
        statements.push(sup_vertical_pos());
        statements.push(table_border_color());
        statements.push(forms_defaults());
        statements.push(button_appearance());
        statements.push(legend_padding());
        statements.push(progress_vertical_alignment());
        statements.push(spin_button_height());
        statements.push(search_appearance());
        statements.push(search_decoration_appearance());
        statements.push(file_upload_button());
        statements.push(summary_display());
    }
}

fn html_defaults() -> CssStatement {
    let default_fonts = "system-ui, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'";

    let selectors_list: CssSelectorsList = "html".into();
    let declarations_block: [CssDeclaration; 4] = [
        CssFontFamily::<()>::new(default_fonts).into(),
        CssLineHeight::<()>::new("1.15").into(),
        ("-webkit-text-size-adjust", "100%").into(),
        CssTabSize::<()>::new("4").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn body_defaults() -> CssStatement {
    let selectors_list: CssSelectorsList = "body".into();
    let declarations_block: CssDeclaration = CssMargin::<()>::new("0").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn b_strong_font_weight() -> CssStatement {
    let selectors_list: CssSelectorsList = ["b", "strong"].into();
    let declarations_block: CssDeclaration = CssFontWeight::<Bolder>::from_recipe().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn monospace_defaults() -> CssStatement {
    let default_fonts =
        "ui-monospace, SFMono-Regular, Consolas, 'Liberation Mono', Menlo, monospace";

    let selectors_list: CssSelectorsList = ["code", "kbd", "samp", "pre"].into();
    let declarations_block: [CssDeclaration; 2] = [
        CssFontFamily::<()>::new(default_fonts).into(),
        CssFontSize::<()>::new("1em").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn small_font_size() -> CssStatement {
    let selectors_list: CssSelectorsList = "small".into();
    let declarations_block: CssDeclaration = CssFontSize::<()>::new("80%").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn sub_sup_defaults() -> CssStatement {
    let selectors_list: CssSelectorsList = ["sub", "sup"].into();
    let declarations_block: [CssDeclaration; 4] = [
        CssFontSize::<()>::new("75%").into(),
        CssLineHeight::<()>::new("0").into(),
        CssPosition::<Relative>::from_recipe().into(),
        CssVerticalAlign::<Baseline>::from_recipe().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn sub_vertical_pos() -> CssStatement {
    let selectors_list: CssSelectorsList = "sub".into();
    let declarations_block: CssDeclaration = CssBottom::<()>::new("-0.25em").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn sup_vertical_pos() -> CssStatement {
    let selectors_list: CssSelectorsList = "sup".into();
    let declarations_block: CssDeclaration = CssTop::<()>::new("-0.5em").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn table_border_color() -> CssStatement {
    let selectors_list: CssSelectorsList = "table".into();
    let declarations_block: CssDeclaration = CssBorderColor::<Currentcolor>::from_recipe().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn forms_defaults() -> CssStatement {
    let selectors_list: CssSelectorsList =
        ["button", "input", "optgroup", "select", "textarea"].into();
    let declarations_block: [CssDeclaration; 4] = [
        CssFontFamily::<Inherit>::from_recipe().into(),
        CssFontSize::<()>::new("100%").into(),
        CssLineHeight::<()>::new("1.15").into(),
        CssMargin::<()>::new("0").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn button_appearance() -> CssStatement {
    let selectors_list: CssSelectorsList = [
        "button",
        "[type=\"button\"]",
        "[type=\"reset\"]",
        "[type=\"submit\"]",
    ]
    .into();
    let declarations_block: CssDeclaration = ("-webkit-appearance", "button").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn legend_padding() -> CssStatement {
    let selectors_list: CssSelectorsList = "legend".into();
    let declarations_block: CssDeclaration = CssPadding::<()>::new("0").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn progress_vertical_alignment() -> CssStatement {
    let selectors_list: CssSelectorsList = "progress".into();
    let declarations_block: CssDeclaration = CssVerticalAlign::<Baseline>::from_recipe().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn spin_button_height() -> CssStatement {
    let selectors_list: CssSelectorsList =
        ["::-webkit-inner-spin-button", "::-webkit-outer-spin-button"].into();
    let declarations_block: CssDeclaration = CssHeight::<Auto>::from_recipe().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn search_appearance() -> CssStatement {
    let selectors_list: CssSelectorsList = r#"[type="search"]"#.into();
    let declarations_block: [CssDeclaration; 2] = [
        ("-webkit-appearance", "textfield").into(),
        CssOutlineOffset::<()>::new("-2px").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn search_decoration_appearance() -> CssStatement {
    let selectors_list: CssSelectorsList = "::-webkit-search-decoration".into();
    let declarations_block: CssDeclaration = ("-webkit-appearance", "none").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn file_upload_button() -> CssStatement {
    let selectors_list: CssSelectorsList = "::-webkit-file-upload-button".into();
    let declarations_block: [CssDeclaration; 2] = [
        ("-webkit-appearance", "button").into(),
        CssFont::<Inherit>::from_recipe().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn summary_display() -> CssStatement {
    let selectors_list: CssSelectorsList = "summary".into();
    let declarations_block: CssDeclaration = CssDisplay::<ListItem>::from_recipe().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}
