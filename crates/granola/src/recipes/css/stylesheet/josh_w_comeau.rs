//! Based on "A Modern CSS Reset" by Josh W. Comeau
//! Source: https://www.joshwcomeau.com/css/custom-css-reset/
//! Released to the public domain by the author

use crate::{macros::declarations_block, prelude::*, recipes::*};

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
            CssRule::new()
                .selectors_list("*:not(dialog)")
                .push_property((Margin, "0")),
            CssAtRule::new()
                .identifier("media")
                .rule("(prefers-reduced-motion: no-preference)")
                .block(
                    CssRule::new()
                        .selectors_list("html")
                        .push_property((InterpolateSize, "allow-keywords")),
                ),
            CssRule::new()
                .selectors_list("body")
                .content(declarations_block![
                    (LineHeight, "1.5"),
                    ("-webkit-font-smoothing", "antialiased"),
                ]),
            CssRule::from(MediaSelectors)
                .push_property(CssDeclaration::from(Display).content("block"))
                .push_property(CssDeclaration::from(MaxWidth).content("100%")),
            CssRule::from(FormControls).push_property((Font, "inherit")),
            CssRule::from(AllHeadingsExt).push_property((OverflowWrap, "break-word")),
            CssRule::new()
                .selectors_list("p")
                .push_property((TextWrap, "pretty")),
            CssRule::from(AllHeadings).push_property((TextWrap, "balance")),
            CssRule::new()
                .selectors_list(bake_comma!["#root", "#__next"])
                .push_property((Isolation, "isolate")),
        ]
    }
}
