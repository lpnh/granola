//! Based on "A (more) Modern CSS Reset" by Andy Bell
//! Source: https://piccalil.li/blog/a-more-modern-css-reset/
//! Licensed under CC BY 3.0 (https://creativecommons.org/licenses/by/3.0/)

use crate::{prelude::*, recipes::*};

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
    fn statements_recipe() -> Bake {
        bake_ws![
            CssRule::from(BoxSizingReset),
            text_size_adjust_reset(),
            default_margin_reset(),
            list_style_reset(),
            body_defaults(),
            headings_and_forms_line_height(),
            headings_text_wrap(),
            CssRule::from(AnchorDefaults),
            images_width_and_display(),
            form_controls_font_inherit(),
            textarea_min_height(),
            target_scroll_margin(),
        ]
    }
}

fn text_size_adjust_reset() -> CssRule {
    let declarations_block = bake_ws![
        CssDeclaration::from(("-moz-text-size-adjust", "none")),
        CssWebkitTextSizeAdjust::from(None),
        CssTextSizeAdjust::from(None),
    ];

    CssRule::new()
        .selectors_list("html")
        .declarations_block(declarations_block)
}

fn default_margin_reset() -> CssRule {
    let selectors_list = bake_comma![
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
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(CssMarginBlockEnd::new().content("0"))
}

fn list_style_reset() -> CssRule {
    CssRule::new()
        .selectors_list(bake_comma!["ul[role='list']", "ol[role='list']"])
        .declarations_block(CssListStyle::from(None))
}

fn body_defaults() -> CssRule {
    let declarations_block = bake_ws![
        CssMinHeight::new().content("100vh"),
        CssLineHeight::new().content("1.5"),
    ];

    CssRule::new()
        .selectors_list("body")
        .declarations_block(declarations_block)
}

fn headings_and_forms_line_height() -> CssRule<Headings> {
    CssRule::from(Headings)
        .push_selectors_list(bake_comma!["button", "input", "label"])
        .declarations_block(CssLineHeight::new().content("1.1"))
}

fn headings_text_wrap() -> CssRule<Headings> {
    CssRule::from(Headings).declarations_block(CssTextWrap::from(Balance))
}

fn images_width_and_display() -> CssRule {
    let declarations_block = bake_ws![CssMaxWidth::new().content("100%"), CssDisplay::from(Block),];

    CssRule::new()
        .selectors_list(bake_comma!["img", "picture"])
        .declarations_block(declarations_block)
}

fn form_controls_font_inherit() -> CssRule<FormControls> {
    let declarations_block = bake_ws![CssFontFamily::from(Inherit), CssFontSize::from(Inherit),];

    CssRule::from(FormControls).declarations_block(declarations_block)
}

fn textarea_min_height() -> CssRule {
    CssRule::new()
        .selectors_list("textarea:not([rows])")
        .declarations_block(CssMinHeight::new().content("10em"))
}

fn target_scroll_margin() -> CssRule {
    CssRule::new()
        .selectors_list(":target")
        .declarations_block(CssScrollMarginBlock::new().content("5ex"))
}
