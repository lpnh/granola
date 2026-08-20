//! Based on "A Modern CSS Reset" by Josh W. Comeau
//! Source: https://www.joshwcomeau.com/css/custom-css-reset/
//! Released to the public domain by the author

use crate::{macros::*, prelude::*, recipes::*};

/// The "Custom CSS Reset" stylesheet recipe.
///
/// [Josh W. Comeau's post with source code](https://www.joshwcomeau.com/css/custom-css-reset/)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let stylesheet = CssStylesheet::from(JoshWComeau);
///
/// assert_eq!(
///     stylesheet.bake_pretty(),
///     r##"*, ::after, ::before {
///   box-sizing: border-box;
/// }
/// *:not(dialog) {
///   margin: 0;
/// }
/// @media (prefers-reduced-motion: no-preference) {
///   html {
///     interpolate-size: allow-keywords;
///   }
/// }
/// body {
///   line-height: 1.5;
///   -webkit-font-smoothing: antialiased;
/// }
/// canvas, img, picture, svg, video {
///   display: block;
///   max-width: 100%;
/// }
/// button, input, select, textarea {
///   font: inherit;
/// }
/// p, h1, h2, h3, h4, h5, h6 {
///   overflow-wrap: break-word;
/// }
/// p {
///   text-wrap: pretty;
/// }
/// h1, h2, h3, h4, h5, h6 {
///   text-wrap: balance;
/// }
/// #root, #__next {
///   isolation: isolate;
/// }
/// "##
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct JoshWComeau;

impl StylesheetRecipe for JoshWComeau {
    fn content_recipe() -> Bake {
        bake_ws![
            CssRule::from(BoxSizingReset),
            rule!("*:not(dialog)", CssDeclaration::from(Margin).value("0")),
            CssAtRule::new()
                .identifier("media")
                .rule("(prefers-reduced-motion: no-preference)")
                .block(rule!(
                    "html",
                    CssDeclaration::from(InterpolateSize).value("allow-keywords")
                )),
            rule!(
                "body",
                declarations_block![
                    CssDeclaration::from(LineHeight).value("1.5"),
                    ("-webkit-font-smoothing", "antialiased"),
                ]
            ),
            CssRule::from(MediaSelectors).content(declarations_block![
                CssDeclaration::from(Display).value("block"),
                CssDeclaration::from(MaxWidth).value("100%")
            ]),
            CssRule::from(FormControls).content(CssDeclaration::from(Font).value("inherit")),
            CssRule::from(AllHeadingsExt)
                .content(CssDeclaration::from(OverflowWrap).value("break-word")),
            rule!("p", CssDeclaration::from(TextWrap).value("pretty")),
            CssRule::from(AllHeadings).content(CssDeclaration::from(TextWrap).value("balance")),
            rule!(
                bake_comma!["#root", "#__next"],
                CssDeclaration::from(Isolation).value("isolate")
            ),
        ]
    }
}
