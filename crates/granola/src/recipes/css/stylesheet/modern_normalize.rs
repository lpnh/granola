//! Based on modern-normalize v3.0.1 by Sindre Sorhus
//! Source: https://github.com/sindresorhus/modern-normalize
//! Licensed under MIT License (https://github.com/sindresorhus/modern-normalize/blob/main/license)

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
///     stylesheet.bake_pretty(),
///     r#"*, ::after, ::before {
///   box-sizing: border-box;
/// }
/// html {
///   font-family:
///     system-ui,
///     "Segoe UI",
///     Roboto,
///     Helvetica,
///     Arial,
///     sans-serif,
///     "Apple Color Emoji",
///     "Segoe UI Emoji";
///   line-height: 1.15;
///   -webkit-text-size-adjust: 100%;
///   tab-size: 4;
/// }
/// body {
///   margin: 0;
/// }
/// b, strong {
///   font-weight: bolder;
/// }
/// code, kbd, samp, pre {
///   font-family:
///     ui-monospace,
///     SFMono-Regular,
///     Consolas,
///     "Liberation Mono",
///     Menlo,
///     monospace;
///   font-size: 1em;
/// }
/// small {
///   font-size: 80%;
/// }
/// sub, sup {
///   font-size: 75%;
///   line-height: 0;
///   position: relative;
///   vertical-align: baseline;
/// }
/// sub {
///   bottom: -0.25em;
/// }
/// sup {
///   top: -0.5em;
/// }
/// table {
///   border-color: currentcolor;
/// }
/// button, input, optgroup, select, textarea {
///   font-family: inherit;
///   font-size: 100%;
///   line-height: 1.15;
///   margin: 0;
/// }
/// button, [type="button"], [type="reset"], [type="submit"] {
///   -webkit-appearance: button;
/// }
/// legend {
///   padding: 0;
/// }
/// progress {
///   vertical-align: baseline;
/// }
/// ::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
///   height: auto;
/// }
/// [type="search"] {
///   -webkit-appearance: textfield;
///   outline-offset: -2px;
/// }
/// ::-webkit-search-decoration {
///   -webkit-appearance: none;
/// }
/// ::-webkit-file-upload-button {
///   -webkit-appearance: button;
///   font: inherit;
/// }
/// summary {
///   display: list-item;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ModernNormalize;

impl StylesheetRecipe for ModernNormalize {
    fn statements_recipe() -> Bake {
        bake_ws![
            CssRule::from(BoxSizingReset),
            html_defaults(),
            body_defaults(),
            CssRule::from(BStrongFontWeight),
            monospace_defaults(),
            CssRule::from(SmallFontSize),
            CssRule::from(SubSupDefaults),
            CssRule::from(SubVerticalPos),
            CssRule::from(SupVerticalPos),
            table_border_color(),
            forms_defaults(),
            button_appearance(),
            legend_padding(),
            CssRule::from(ProgressVerticalAlignment),
            CssRule::from(SpinButtonHeight),
            search_appearance(),
            CssRule::from(SearchDecorationAppearance),
            file_upload_button(),
            CssRule::from(SummaryDisplay),
        ]
    }
}

fn html_defaults() -> CssRule {
    let default_fonts = "system-ui, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'";

    let declarations_block = bake_ws![
        CssFontFamily::new().content(default_fonts),
        CssLineHeight::new().content("1.15"),
        CssWebkitTextSizeAdjust::new().content("100%"),
        CssTabSize::new().content("4"),
    ];

    CssRule::new()
        .selectors_list("html")
        .declarations_block(declarations_block)
}

fn body_defaults() -> CssRule {
    CssRule::new()
        .push_selector("body")
        .push_property(CssMargin::new().content("0"))
}

fn monospace_defaults() -> CssRule<MonospaceSelectors> {
    let default_fonts =
        "ui-monospace, SFMono-Regular, Consolas, 'Liberation Mono', Menlo, monospace";

    CssRule::from(MonospaceSelectors)
        .push_property(CssFontFamily::new().content(default_fonts))
        .push_property(CssFontSize::new().content("1em"))
}

fn table_border_color() -> CssRule {
    CssRule::new()
        .selectors_list("table")
        .declarations_block(CssBorderColor::from(Currentcolor))
}

fn forms_defaults() -> CssRule<FormControlsExt> {
    let declarations_block = bake_ws![
        CssFontFamily::from(Inherit),
        CssFontSize::new().content("100%"),
        CssLineHeight::new().content("1.15"),
        CssMargin::new().content("0"),
    ];

    CssRule::from(FormControlsExt).declarations_block(declarations_block)
}

fn button_appearance() -> CssRule {
    let selectors_list = bake_comma![
        "button",
        r#"[type="button"]"#,
        r#"[type="reset"]"#,
        r#"[type="submit"]"#,
    ];
    let declarations_block = CssDeclaration::from(("-webkit-appearance", "button"));

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
}

fn legend_padding() -> CssRule {
    CssRule::new()
        .selectors_list("legend")
        .declarations_block(CssPadding::new().content("0"))
}

fn search_appearance() -> CssRule {
    CssRule::new()
        .selectors_list(r#"[type="search"]"#)
        .push_property(CssDeclaration::from(("-webkit-appearance", "textfield")))
        .push_property(CssOutlineOffset::new().content("-2px"))
}

fn file_upload_button() -> CssRule {
    let declarations_block = bake_ws![
        CssDeclaration::from(("-webkit-appearance", "button")),
        CssFont::from(Inherit),
    ];

    CssRule::new()
        .selectors_list("::-webkit-file-upload-button")
        .declarations_block(declarations_block)
}
