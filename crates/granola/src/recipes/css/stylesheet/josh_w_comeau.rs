//! Based on "A Modern CSS Reset" by Josh W. Comeau
//! Source: https://www.joshwcomeau.com/css/custom-css-reset/
//! Released to the public domain by the author

use crate::{prelude::*, recipes::*};

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
    fn statements_recipe() -> Bake {
        bake_ws![
            CssRule::from(BoxSizingReset),
            default_margin_reset(),
            keyword_animations(),
            body_defaults(),
            media_defaults(),
            form_controls_font_inherit(),
            text_overflow_reset(),
            paragraph_text_wrap(),
            headings_text_wrap(),
            root_stacking_context(),
        ]
    }
}

fn default_margin_reset() -> CssRule {
    CssRule::new()
        .selectors_list("*:not(dialog)")
        .declarations_block(CssMargin::new().content("0"))
}

fn keyword_animations() -> CssAtRule {
    let inner_rule = CssRule::new()
        .selectors_list("html")
        .declarations_block(CssInterpolateSize::from(AllowKeywords));

    CssAtRule::new()
        .identifier("media")
        .rule("(prefers-reduced-motion: no-preference)")
        .block(inner_rule)
}

fn body_defaults() -> CssRule {
    let declarations_block = bake_ws![
        CssLineHeight::new().content("1.5"),
        CssDeclaration::from(("-webkit-font-smoothing", "antialiased")),
    ];

    CssRule::new()
        .selectors_list("body")
        .declarations_block(declarations_block)
}

fn media_defaults() -> CssRule<MediaSelectors> {
    CssRule::from(MediaSelectors)
        .push_property(CssDisplay::from(Block))
        .push_property(CssMaxWidth::new().content("100%"))
}

fn form_controls_font_inherit() -> CssRule<FormControls> {
    let declarations_block = CssFont::from(Inherit);

    CssRule::from(FormControls).declarations_block(declarations_block)
}

fn text_overflow_reset() -> CssRule<AllHeadingsExt> {
    CssRule::from(AllHeadingsExt).declarations_block(CssOverflowWrap::from(BreakWord))
}

fn paragraph_text_wrap() -> CssRule {
    CssRule::new()
        .selectors_list("p")
        .declarations_block(CssTextWrap::from(Pretty))
}

fn headings_text_wrap() -> CssRule<AllHeadings> {
    CssRule::from(AllHeadings).declarations_block(CssTextWrap::from(Balance))
}

fn root_stacking_context() -> CssRule {
    CssRule::new()
        .selectors_list(bake_comma!["#root", "#__next"])
        .declarations_block(CssIsolation::from(Isolate))
}
