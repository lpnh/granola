// Based on modern-normalize v3.0.1 by Sindre Sorhus
// Source: https://github.com/sindresorhus/modern-normalize
// Licensed under MIT License (https://github.com/sindresorhus/modern-normalize/blob/main/license)

use crate::{prelude::*, recipes::*};

/// The modern-normalize stylesheet recipe.
///
/// [modern-normalize source code](https://github.com/sindresorhus/modern-normalize/blob/main/modern-normalize.css)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let stylesheet = CssStylesheet::from(ModernNormalize);
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
            CssRule::from(BoxSizingReset).into(),
            html_defaults(),
            body_defaults(),
            CssRule::from(BStrongFontWeight).into(),
            monospace_defaults(),
            CssRule::from(SmallFontSize).into(),
            CssRule::from(SubSupDefaults).into(),
            CssRule::from(SubVerticalPos).into(),
            CssRule::from(SupVerticalPos).into(),
            table_border_color(),
            forms_defaults(),
            button_appearance(),
            legend_padding(),
            CssRule::from(ProgressVerticalAlignment).into(),
            CssRule::from(SpinButtonHeight).into(),
            search_appearance(),
            CssRule::from(SearchDecorationAppearance).into(),
            file_upload_button(),
            CssRule::from(SummaryDisplay).into(),
        ]);
    }
}

fn html_defaults() -> CssStatement {
    let default_fonts = "system-ui, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'";

    let selectors_list = "html";
    let declarations_block: [CssDeclaration; 4] = [
        CssFontFamily::new().content(default_fonts).into(),
        CssLineHeight::new().content("1.15").into(),
        CssWebkitTextSizeAdjust::new().content("100%").into(),
        CssTabSize::new().content("4").into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn body_defaults() -> CssStatement {
    CssRule::new()
        .push_selector("body")
        .push_property(CssMargin::new().content("0"))
        .into()
}

fn monospace_defaults() -> CssStatement {
    let default_fonts =
        "ui-monospace, SFMono-Regular, Consolas, 'Liberation Mono', Menlo, monospace";

    let selectors_list = CssSelectorsList::from(MonospaceSelectors).bake_recipe();
    let declarations_block: [CssDeclaration; 2] = [
        CssFontFamily::new().content(default_fonts).into(),
        CssFontSize::new().content("1em").into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn table_border_color() -> CssStatement {
    CssRule::new()
        .selectors_list("table")
        .declarations_block(CssBorderColor::from(Currentcolor))
        .into()
}

fn forms_defaults() -> CssStatement {
    let selectors_list = CssSelectorsList::from(FormControlsExt).bake_recipe();
    let declarations_block: [CssDeclaration; 4] = [
        CssFontFamily::from(Inherit).into(),
        CssFontSize::new().content("100%").into(),
        CssLineHeight::new().content("1.15").into(),
        CssMargin::new().content("0").into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn button_appearance() -> CssStatement {
    let selectors_list = [
        "button",
        "[type=\"button\"]",
        "[type=\"reset\"]",
        "[type=\"submit\"]",
    ];
    let declarations_block = ("-webkit-appearance", "button");

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn legend_padding() -> CssStatement {
    CssRule::new()
        .selectors_list("legend")
        .declarations_block(CssPadding::new().content("0"))
        .into()
}

fn search_appearance() -> CssStatement {
    let selectors_list = r#"[type="search"]"#;
    let declarations_block: [CssDeclaration; 2] = [
        ("-webkit-appearance", "textfield").into(),
        CssOutlineOffset::new().content("-2px").into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn file_upload_button() -> CssStatement {
    let selectors_list = "::-webkit-file-upload-button";
    let declarations_block: [CssDeclaration; 2] = [
        ("-webkit-appearance", "button").into(),
        CssFont::from(Inherit).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}
