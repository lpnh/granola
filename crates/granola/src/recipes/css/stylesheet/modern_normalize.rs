//! Based on modern-normalize v3.0.1 by Sindre Sorhus
//! Source: https://github.com/sindresorhus/modern-normalize
//! Licensed under MIT License (https://github.com/sindresorhus/modern-normalize/blob/main/license)

use crate::{macros::*, prelude::*, recipes::*};

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
    fn content_recipe() -> Bake {
        rules![
            BoxSizingReset,
            rule!(
                "html",
                declarations_block![
                    CssDeclaration::from(FontFamily).value(
                        "system-ui, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'"
                    ),
                    CssDeclaration::from(LineHeight).value("1.15"),
                    CssDeclaration::from(WebkitTextSizeAdjust).value("100%"),
                    CssDeclaration::from(TabSize).value("4"),
                ]
            ),
            rule!("body", CssDeclaration::from(Margin).value("0")),
            BStrongFontWeight,
            CssRule::from(MonospaceSelectors).content(declarations_block![
                CssDeclaration::from(FontFamily).value(
                    "ui-monospace, SFMono-Regular, Consolas, 'Liberation Mono', Menlo, monospace"
                ),
                CssDeclaration::from(FontSize).value("1em")
            ]),
            SmallFontSize,
            SubSupDefaults,
            SubVerticalPos,
            SupVerticalPos,
            rule!(
                "table",
                CssDeclaration::from(BorderColor).value("currentcolor")
            ),
            CssRule::from(FormControlsExt).content(declarations_block![
                CssDeclaration::from(FontFamily).value("inherit"),
                CssDeclaration::from(FontSize).value("100%"),
                CssDeclaration::from(LineHeight).value("1.15"),
                CssDeclaration::from(Margin).value("0"),
            ]),
            rule!(
                bake_comma![
                    "button",
                    r#"[type="button"]"#,
                    r#"[type="reset"]"#,
                    r#"[type="submit"]"#,
                ],
                declaration!("-webkit-appearance", "button")
            ),
            rule!("legend", CssDeclaration::from(Padding).value("0")),
            ProgressVerticalAlignment,
            SpinButtonHeight,
            rule!(
                r#"[type="search"]"#,
                declarations_block![
                    ("-webkit-appearance", "textfield"),
                    CssDeclaration::from(OutlineOffset).value("-2px")
                ]
            ),
            SearchDecorationAppearance,
            rule!(
                "::-webkit-file-upload-button",
                declarations_block![
                    ("-webkit-appearance", "button"),
                    CssDeclaration::from(Font).value("inherit")
                ]
            ),
            SummaryDisplayListItem,
        ]
    }
}
