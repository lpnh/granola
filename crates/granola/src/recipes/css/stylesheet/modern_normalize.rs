// Based on modern-normalize v3.0.1 by Sindre Sorhus
// Source: https://github.com/sindresorhus/modern-normalize
// Licensed under MIT License (https://github.com/sindresorhus/modern-normalize/blob/main/license)

use crate::{recipes::*, prelude::*};

/// The modern-normalize stylesheet recipe.
///
/// [modern-normalize source code](https://github.com/sindresorhus/modern-normalize/blob/main/modern-normalize.css)
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let stylesheet: CssStylesheet<ModernNormalize> = CssStylesheet::from_cookbook();
///
/// assert_eq!(
///     stylesheet.bake(),
///     r#"*,
/// ::after,
/// ::before {
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
/// b,
/// strong {
///   font-weight: bolder;
/// }
///
/// code,
/// kbd,
/// samp,
/// pre {
///   font-family: ui-monospace, SFMono-Regular, Consolas, 'Liberation Mono', Menlo, monospace;
///   font-size: 1em;
/// }
///
/// small {
///   font-size: 80%;
/// }
///
/// sub,
/// sup {
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
/// button,
/// input,
/// optgroup,
/// select,
/// textarea {
///   font-family: inherit;
///   font-size: 100%;
///   line-height: 1.15;
///   margin: 0;
/// }
///
/// button,
/// [type="button"],
/// [type="reset"],
/// [type="submit"] {
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
/// ::-webkit-inner-spin-button,
/// ::-webkit-outer-spin-button {
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
        statements.extend([
            CssRule::<BoxSizingReset>::from_cookbook().into(),
            html_defaults(),
            body_defaults(),
            CssRule::<BStrongFontWeight>::from_cookbook().into(),
            monospace_defaults(),
            CssRule::<SmallFontSize>::from_cookbook().into(),
            CssRule::<SubSupDefaults>::from_cookbook().into(),
            CssRule::<SubVerticalPos>::from_cookbook().into(),
            CssRule::<SupVerticalPos>::from_cookbook().into(),
            table_border_color(),
            forms_defaults(),
            button_appearance(),
            legend_padding(),
            CssRule::<ProgressVerticalAlignment>::from_cookbook().into(),
            CssRule::<SpinButtonHeight>::from_cookbook().into(),
            search_appearance(),
            CssRule::<SearchDecorationAppearance>::from_cookbook().into(),
            file_upload_button(),
            CssRule::<SummaryDisplay>::from_cookbook().into(),
        ]);
    }
}

fn html_defaults() -> CssStatement {
    let default_fonts = "system-ui, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'";

    let selectors_list: CssSelectorsList = "html".into();
    let declarations_block: [CssDeclaration; 4] = [
        CssFontFamily::<()>::new(default_fonts).into(),
        CssLineHeight::<()>::new("1.15").into(),
        CssWebkitTextSizeAdjust::<()>::new("100%").into(),
        CssTabSize::<()>::new("4").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn body_defaults() -> CssStatement {
    let selectors_list: CssSelectorsList = "body".into();
    let declarations_block: CssDeclaration = CssMargin::<()>::new("0").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn monospace_defaults() -> CssStatement {
    let default_fonts =
        "ui-monospace, SFMono-Regular, Consolas, 'Liberation Mono', Menlo, monospace";

    let selectors_list = CssSelectorsList::<MonospaceSelectors>::from_cookbook().bake_recipe();
    let declarations_block: [CssDeclaration; 2] = [
        CssFontFamily::<()>::new(default_fonts).into(),
        CssFontSize::<()>::new("1em").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn table_border_color() -> CssStatement {
    let selectors_list: CssSelectorsList = "table".into();
    let declarations_block: CssDeclaration = CssBorderColor::<Currentcolor>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn forms_defaults() -> CssStatement {
    let selectors_list = CssSelectorsList::<FormControlsExt>::from_cookbook().bake_recipe();
    let declarations_block: [CssDeclaration; 4] = [
        CssFontFamily::<Inherit>::from_cookbook().into(),
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

fn search_appearance() -> CssStatement {
    let selectors_list: CssSelectorsList = r#"[type="search"]"#.into();
    let declarations_block: [CssDeclaration; 2] = [
        ("-webkit-appearance", "textfield").into(),
        CssOutlineOffset::<()>::new("-2px").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn file_upload_button() -> CssStatement {
    let selectors_list: CssSelectorsList = "::-webkit-file-upload-button".into();
    let declarations_block: [CssDeclaration; 2] = [
        ("-webkit-appearance", "button").into(),
        CssFont::<Inherit>::from_cookbook().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}
