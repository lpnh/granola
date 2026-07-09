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
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(BoxSizingReset),
            rule!("*:not(dialog)", declaration!(Margin, "0")),
            CssAtRule::new()
                .identifier("media")
                .rule("(prefers-reduced-motion: no-preference)")
                .block(rule!(
                    "html",
                    declaration!(InterpolateSize, "allow-keywords")
                )),
            rule!(
                "body",
                declarations_block![
                    (LineHeight, "1.5"),
                    ("-webkit-font-smoothing", "antialiased"),
                ]
            ),
            CssRule::from((
                MediaSelectors,
                declarations_block![(Display, "block"), (MaxWidth, "100%")]
            )),
            CssRule::from((FormControls, declaration!(Font, "inherit"))),
            CssRule::from((AllHeadingsExt, declaration!(OverflowWrap, "break-word"))),
            rule!("p", declaration!(TextWrap, "pretty")),
            CssRule::from((AllHeadings, declaration!(TextWrap, "balance"))),
            rule!(
                bake_comma!["#root", "#__next"],
                declaration!(Isolation, "isolate")
            ),
        ]
    }
}
