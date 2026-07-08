//! Based on "A (more) Modern CSS Reset" by Andy Bell
//! Source: https://piccalil.li/blog/a-more-modern-css-reset/
//! Licensed under CC BY 3.0 (https://creativecommons.org/licenses/by/3.0/)

use crate::{macros::declarations_block, prelude::*, recipes::*};

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
///     stylesheet.bake_pretty(),
///     r#"*, ::after, ::before {
///   box-sizing: border-box;
/// }
/// html {
///   -moz-text-size-adjust: none;
///   -webkit-text-size-adjust: none;
///   text-size-adjust: none;
/// }
/// body, h1, h2, h3, h4, p, figure, blockquote, dl, dd {
///   margin-block-end: 0;
/// }
/// ul[role="list"], ol[role="list"] {
///   list-style: none;
/// }
/// body {
///   min-height: 100vh;
///   line-height: 1.5;
/// }
/// h1, h2, h3, h4, button, input, label {
///   line-height: 1.1;
/// }
/// h1, h2, h3, h4 {
///   text-wrap: balance;
/// }
/// a:not([class]) {
///   text-decoration-skip-ink: auto;
///   color: currentcolor;
/// }
/// img, picture {
///   max-width: 100%;
///   display: block;
/// }
/// button, input, select, textarea {
///   font-family: inherit;
///   font-size: inherit;
/// }
/// textarea:not([rows]) {
///   min-height: 10em;
/// }
/// :target {
///   scroll-margin-block: 5ex;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AndyBell;

impl StylesheetRecipe for AndyBell {
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(BoxSizingReset),
            CssRule::new()
                .selectors_list("html")
                .content(declarations_block![
                    ("-moz-text-size-adjust", "none"),
                    (WebkitTextSizeAdjust, "none"),
                    (TextSizeAdjust, "none"),
                ]),
            CssRule::new()
                .selectors_list(bake_comma![
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
                ])
                .push_property((MarginBlockEnd, "0")),
            CssRule::from(ListReset),
            CssRule::new()
                .selectors_list("body")
                .content(declarations_block![
                    (MinHeight, "100vh"),
                    (LineHeight, "1.5")
                ]),
            CssRule::from(Headings)
                .push_selectors_list(bake_comma!["button", "input", "label"])
                .push_property((LineHeight, "1.1")),
            CssRule::from(Headings).push_property((TextWrap, "balance")),
            CssRule::from(AnchorDefaults),
            CssRule::new()
                .selectors_list(bake_comma!["img", "picture"])
                .content(declarations_block![(MaxWidth, "100%"), (Display, "block")]),
            CssRule::from(FormControls).content(declarations_block![
                (FontFamily, "inherit"),
                (FontSize, "inherit")
            ]),
            CssRule::new()
                .selectors_list("textarea:not([rows])")
                .push_property((MinHeight, "10em")),
            CssRule::new()
                .selectors_list(":target")
                .push_property((ScrollMarginBlock, "5ex")),
        ]
    }
}
