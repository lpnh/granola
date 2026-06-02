// Based on "A Modern CSS Reset" by Josh W. Comeau
// Source: https://www.joshwcomeau.com/css/custom-css-reset/
// Released to the public domain by the author

use crate::{cookbook::*, prelude::*};

/// The "Custom CSS Reset" stylesheet recipe.
///
/// [Josh W. Comeau's post with source code](https://www.joshwcomeau.com/css/custom-css-reset/)
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let stylesheet: CssStylesheet<JoshWComeau> = CssStylesheet::from_recipe();
///
/// assert_eq!(
///     stylesheet.bake(),
///     "*,
/// ::after,
/// ::before {
///   box-sizing: border-box;
/// }
///
/// *:not(dialog) {
///   margin: 0;
/// }
///
/// @media (prefers-reduced-motion: no-preference) {
///   html {
///     interpolate-size: allow-keywords;
///   }
/// }
///
/// body {
///   line-height: 1.5;
///   -webkit-font-smoothing: antialiased;
/// }
///
/// canvas,
/// img,
/// picture,
/// svg,
/// video {
///   display: block;
///   max-width: 100%;
/// }
///
/// button,
/// input,
/// select,
/// textarea {
///   font: inherit;
/// }
///
/// p,
/// h1,
/// h2,
/// h3,
/// h4,
/// h5,
/// h6 {
///   overflow-wrap: break-word;
/// }
///
/// p {
///   text-wrap: pretty;
/// }
///
/// h1,
/// h2,
/// h3,
/// h4,
/// h5,
/// h6 {
///   text-wrap: balance;
/// }
///
/// #root,
/// #__next {
///   isolation: isolate;
/// }"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct JoshWComeau;

impl StylesheetRecipe for JoshWComeau {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([
            CssRule::<BoxSizingReset>::from_recipe().into(),
            default_margin_reset(),
            keyword_animations(),
            body_defaults(),
            media_defaults(),
            form_controls_font_inherit(),
            text_overflow_reset(),
            paragraph_text_wrap(),
            headings_text_wrap(),
            root_stacking_context(),
        ]);
    }
}

fn default_margin_reset() -> CssStatement {
    CssRule::<()>::new("*:not(dialog)", CssMargin::<()>::new("0")).into()
}

fn keyword_animations() -> CssStatement {
    let inner_rule = CssRule::<()>::new("html", CssInterpolateSize::<AllowKeywords>::from_recipe());

    CssAtRule::<()>::new("media", "(prefers-reduced-motion: no-preference)")
        .block(inner_rule)
        .into()
}

fn body_defaults() -> CssStatement {
    let declarations_block: [CssDeclaration; 2] = [
        CssLineHeight::<()>::new("1.5").into(),
        ("-webkit-font-smoothing", "antialiased").into(),
    ];

    CssRule::<()>::new("body", declarations_block).into()
}

fn media_defaults() -> CssStatement {
    let selectors_list = CssSelectorsList::<MediaSelectors>::from_recipe().bake_recipe();
    let declarations_block: [CssDeclaration; 2] = [
        CssDisplay::<Block>::from_recipe().into(),
        CssMaxWidth::<()>::new("100%").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn form_controls_font_inherit() -> CssStatement {
    let selectors_list = CssSelectorsList::<FormControls>::from_recipe().bake_recipe();
    let declarations_block = CssFont::<Inherit>::from_recipe();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn text_overflow_reset() -> CssStatement {
    let selectors_list = CssSelectorsList::<AllHeadingsExt>::from_recipe().bake_recipe();
    let declarations_block = CssOverflowWrap::<BreakWord>::from_recipe();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn paragraph_text_wrap() -> CssStatement {
    CssRule::<()>::new("p", CssTextWrap::<Pretty>::from_recipe()).into()
}

fn headings_text_wrap() -> CssStatement {
    let selectors_list = CssSelectorsList::<AllHeadings>::from_recipe().bake_recipe();
    let declarations_block = CssTextWrap::<Balance>::from_recipe();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn root_stacking_context() -> CssStatement {
    let selectors_list: CssSelectorsList = ["#root", "#__next"].into();
    let declarations_block = CssIsolation::<Isolate>::from_recipe();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}
