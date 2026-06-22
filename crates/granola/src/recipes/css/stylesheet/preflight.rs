//! Based on `preflight.css` by Tailwind Labs, Inc.
//! Source: https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/preflight.css
//! Licensed under MIT License (https://github.com/tailwindlabs/tailwindcss/blob/main/LICENSE)

use crate::{prelude::*, recipes::*};

/// The preflight stylesheet recipe.
///
/// [preflight source code](https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/preflight.css)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let stylesheet = CssStylesheet::from(Preflight);
///
/// assert_eq!(
///     stylesheet.bake_pretty(),
///     r#"*, ::after, ::before, ::backdrop, ::file-selector-button {
///   box-sizing: border-box;
///   margin: 0;
///   padding: 0;
///   border: 0 solid;
/// }
/// html, :host {
///   line-height: 1.5;
///   -webkit-text-size-adjust: 100%;
///   tab-size: 4;
///   font-family: --theme(
///     --default-font-family,
///     ui-sans-serif,
///     system-ui,
///     sans-serif,
///     "Apple Color Emoji",
///     "Segoe UI Emoji",
///     "Segoe UI Symbol",
///     "Noto Color Emoji"
///   );
///   font-feature-settings: --theme(--default-font-feature-settings, normal);
///   font-variation-settings: --theme(--default-font-variation-settings, normal);
///   -webkit-tap-highlight-color: transparent;
/// }
/// hr {
///   height: 0;
///   color: inherit;
///   border-top-width: 1px;
/// }
/// abbr:where([title]) {
///   -webkit-text-decoration: underline dotted;
///   text-decoration: underline dotted;
/// }
/// h1, h2, h3, h4, h5, h6 {
///   font-size: inherit;
///   font-weight: inherit;
/// }
/// a {
///   color: inherit;
///   -webkit-text-decoration: inherit;
///   text-decoration: inherit;
/// }
/// b, strong {
///   font-weight: bolder;
/// }
/// code, kbd, samp, pre {
///   font-family: --theme(
///     --default-mono-font-family,
///     ui-monospace,
///     SFMono-Regular,
///     Menlo,
///     Monaco,
///     Consolas,
///     "Liberation Mono",
///     "Courier New",
///     monospace
///   );
///   font-feature-settings: --theme(--default-mono-font-feature-settings, normal);
///   font-variation-settings: --theme(
///     --default-mono-font-variation-settings,
///     normal
///   );
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
///   text-indent: 0;
///   border-color: inherit;
///   border-collapse: collapse;
/// }
/// :-moz-focusring {
///   outline: auto;
/// }
/// progress {
///   vertical-align: baseline;
/// }
/// summary {
///   display: list-item;
/// }
/// ol, ul, menu {
///   list-style: none;
/// }
/// img, svg, video, canvas, audio, iframe, embed, object {
///   display: block;
///   vertical-align: middle;
/// }
/// img, video {
///   max-width: 100%;
///   height: auto;
/// }
/// button, input, optgroup, select, textarea, ::file-selector-button {
///   font: inherit;
///   font-feature-settings: inherit;
///   font-variation-settings: inherit;
///   letter-spacing: inherit;
///   color: inherit;
///   border-radius: 0;
///   background-color: transparent;
///   opacity: 1;
/// }
/// :where(select:is([multiple], [size])) optgroup {
///   font-weight: bolder;
/// }
/// :where(select:is([multiple], [size])) optgroup option {
///   padding-inline-start: 20px;
/// }
/// ::file-selector-button {
///   margin-inline-end: 4px;
/// }
/// ::placeholder {
///   opacity: 1;
/// }
/// @supports (not (-webkit-appearance: -apple-pay-button)) or
///   (contain-intrinsic-size: 1px) {
///   ::placeholder {
///     color: color-mix(in oklab, currentcolor 50%, transparent);
///   }
/// }
/// textarea {
///   resize: vertical;
/// }
/// ::-webkit-search-decoration {
///   -webkit-appearance: none;
/// }
/// ::-webkit-date-and-time-value {
///   min-height: 1lh;
///   text-align: inherit;
/// }
/// ::-webkit-datetime-edit {
///   display: inline flex;
/// }
/// ::-webkit-datetime-edit-fields-wrapper {
///   padding: 0;
/// }
/// ::-webkit-datetime-edit,
/// ::-webkit-datetime-edit-year-field,
/// ::-webkit-datetime-edit-month-field,
/// ::-webkit-datetime-edit-day-field,
/// ::-webkit-datetime-edit-hour-field,
/// ::-webkit-datetime-edit-minute-field,
/// ::-webkit-datetime-edit-second-field,
/// ::-webkit-datetime-edit-millisecond-field,
/// ::-webkit-datetime-edit-meridiem-field {
///   padding-block: 0;
/// }
/// ::-webkit-calendar-picker-indicator {
///   line-height: 1;
/// }
/// :-moz-ui-invalid {
///   box-shadow: none;
/// }
/// button,
/// input:where([type="button"], [type="reset"], [type="submit"]),
/// ::file-selector-button {
///   appearance: button;
/// }
/// ::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
///   height: auto;
/// }
/// [hidden]:where(:not([hidden="until-found"])) {
///   display: none !important;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Preflight;

impl StylesheetRecipe for Preflight {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([
            CssRule::from(UniversalReset).into(),
            html_host_defaults(),
            CssRule::from(HrReset).into(),
            abbr_text_decoration(),
            CssRule::from(AllHeadingsFontReset).into(),
            CssRule::from(AnchorInherit).into(),
            CssRule::from(BStrongFontWeight).into(),
            monospace_defaults(),
            CssRule::from(SmallFontSize).into(),
            CssRule::from(SubSupDefaults).into(),
            CssRule::from(SubVerticalPos).into(),
            CssRule::from(SupVerticalPos).into(),
            table_reset(),
            moz_focusring_outline(),
            CssRule::from(ProgressVerticalAlignment).into(),
            CssRule::from(SummaryDisplay).into(),
            list_reset(),
            replaced_element_display(),
            replaced_element_sizing(),
            form_controls_reset(),
            optgroup_font_weight(),
            optgroup_option_indent(),
            file_selector_button_spacing(),
            placeholder_opacity(),
            placeholder_color_supports(),
            textarea_resize(),
            CssRule::from(SearchDecorationAppearance).into(),
            date_and_time_value(),
            datetime_edit_display(),
            datetime_edit_fields_wrapper(),
            datetime_edit_padding_block(),
            calendar_picker_indicator(),
            moz_ui_invalid_box_shadow(),
            button_appearance(),
            CssRule::from(SpinButtonHeight).into(),
            hidden_display(),
        ]);
    }
}

fn html_host_defaults() -> CssStatement {
    let sans_font_family = "--theme(
  --default-font-family,
  ui-sans-serif,
  system-ui,
  sans-serif,
  'Apple Color Emoji',
  'Segoe UI Emoji',
  'Segoe UI Symbol',
  'Noto Color Emoji'
)";

    let selectors_list = ["html", ":host"];
    let declarations_block: [CssDeclaration; 7] = [
        CssLineHeight::new().content("1.5").into(),
        CssWebkitTextSizeAdjust::new().content("100%").into(),
        CssTabSize::new().content("4").into(),
        CssFontFamily::new().content(sans_font_family).into(),
        CssFontFeatureSettings::new()
            .content("--theme(--default-font-feature-settings, normal)")
            .into(),
        CssFontVariationSettings::new()
            .content("--theme(--default-font-variation-settings, normal)")
            .into(),
        ("-webkit-tap-highlight-color", "transparent").into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn abbr_text_decoration() -> CssStatement {
    let selectors_list = "abbr:where([title])";
    let declarations_block: [CssDeclaration; 2] = [
        CssWebkitTextDecoration::from((Underline, Dotted)).into(),
        CssTextDecoration::from((Underline, Dotted))
            .bake_recipe()
            .into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn monospace_defaults() -> CssStatement {
    let mono_font_family = "--theme(
  --default-mono-font-family,
  ui-monospace,
  SFMono-Regular,
  Menlo,
  Monaco,
  Consolas,
  'Liberation Mono',
  'Courier New',
  monospace
)";

    let selectors_list = CssSelectorsList::from(MonospaceSelectors).bake_recipe();
    let declarations_block: [CssDeclaration; 4] = [
        CssFontFamily::new().content(mono_font_family).into(),
        CssFontFeatureSettings::new()
            .content("--theme(--default-mono-font-feature-settings, normal)")
            .into(),
        CssFontVariationSettings::new()
            .content("--theme(--default-mono-font-variation-settings, normal)")
            .into(),
        CssFontSize::new().content("1em").into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn table_reset() -> CssStatement {
    let selectors_list = "table";
    let declarations_block: [CssDeclaration; 3] = [
        CssTextIndent::new().content("0").into(),
        CssBorderColor::from(Inherit).into(),
        CssBorderCollapse::from(Collapse).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn moz_focusring_outline() -> CssStatement {
    CssRule::new()
        .selectors_list(":-moz-focusring")
        .declarations_block(CssOutline::from(Auto))
        .into()
}

fn list_reset() -> CssStatement {
    let selectors_list = ["ol", "ul", "menu"];
    let declarations_block = CssListStyle::from(None);

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn replaced_element_display() -> CssStatement {
    let selectors_list = [
        "img", "svg", "video", "canvas", "audio", "iframe", "embed", "object",
    ];
    let declarations_block: [CssDeclaration; 2] = [
        CssDisplay::from(Block).into(),
        CssVerticalAlign::from(Middle).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn replaced_element_sizing() -> CssStatement {
    let selectors_list = ["img", "video"];
    let declarations_block: [CssDeclaration; 2] = [
        CssMaxWidth::new().content("100%").into(),
        CssHeight::from(Auto).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn form_controls_reset() -> CssStatement {
    let selectors_list = CssSelectorsList::from(FormControlsExt)
        .push(CssSimpleSelector::from(UniversalFileSelectorButton))
        .bake_recipe();
    let declarations_block: [CssDeclaration; 8] = [
        CssFont::from(Inherit).into(),
        CssFontFeatureSettings::from(Inherit).into(),
        CssFontVariationSettings::from(Inherit).into(),
        CssLetterSpacing::from(Inherit).into(),
        CssColor::from(Inherit).into(),
        CssBorderRadius::new().content("0").into(),
        CssBackgroundColor::from(Transparent).into(),
        CssOpacity::new().content("1").into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn optgroup_font_weight() -> CssStatement {
    let selectors_list = CssSimpleSelector::new()
        .selector(":where(select:is([multiple], [size]))")
        .descendant("optgroup");
    let declarations_block = CssFontWeight::from(Bolder).bake_recipe();

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn optgroup_option_indent() -> CssStatement {
    let selectors_list = CssSimpleSelector::new()
        .selector(":where(select:is([multiple], [size]))")
        .descendant("optgroup")
        .descendant("option");
    let declarations_block = CssPaddingInlineStart::new().content("20px");

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn file_selector_button_spacing() -> CssStatement {
    CssRule::new()
        .selectors_list(CssSimpleSelector::from(UniversalFileSelectorButton))
        .declarations_block(CssMarginInlineEnd::new().content("4px"))
        .into()
}

fn placeholder_opacity() -> CssStatement {
    CssRule::new()
        .selectors_list(CssSimpleSelector::from(UniversalPlaceholder))
        .declarations_block(CssOpacity::new().content("1"))
        .into()
}

fn placeholder_color_supports() -> CssStatement {
    let condition = "(not (-webkit-appearance: -apple-pay-button)) or
  (contain-intrinsic-size: 1px)";

    let placeholder = CssRule::new()
        .selectors_list(CssSimpleSelector::from(UniversalPlaceholder))
        .declarations_block(
            CssColor::new().content("color-mix(in oklab, currentcolor 50%, transparent)"),
        );

    CssAtRule::new()
        .identifier("supports")
        .rule(condition)
        .block(placeholder)
        .into()
}

fn textarea_resize() -> CssStatement {
    CssRule::new()
        .selectors_list("textarea")
        .declarations_block(CssResize::from(Vertical))
        .into()
}

fn date_and_time_value() -> CssStatement {
    let selectors_list = "::-webkit-date-and-time-value";
    let declarations_block: [CssDeclaration; 2] = [
        CssMinHeight::new().content("1lh").into(),
        CssTextAlign::from(Inherit).into(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn datetime_edit_display() -> CssStatement {
    CssRule::new()
        .selectors_list("::-webkit-datetime-edit")
        .declarations_block(CssDisplay::from((Inline, Flex)))
        .into()
}

fn datetime_edit_fields_wrapper() -> CssStatement {
    CssRule::new()
        .selectors_list("::-webkit-datetime-edit-fields-wrapper")
        .declarations_block(CssPadding::new().content("0"))
        .into()
}

fn datetime_edit_padding_block() -> CssStatement {
    let selectors_list = [
        "::-webkit-datetime-edit",
        "::-webkit-datetime-edit-year-field",
        "::-webkit-datetime-edit-month-field",
        "::-webkit-datetime-edit-day-field",
        "::-webkit-datetime-edit-hour-field",
        "::-webkit-datetime-edit-minute-field",
        "::-webkit-datetime-edit-second-field",
        "::-webkit-datetime-edit-millisecond-field",
        "::-webkit-datetime-edit-meridiem-field",
    ];
    let declarations_block = CssPaddingBlock::new().content("0");

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn calendar_picker_indicator() -> CssStatement {
    CssRule::new()
        .selectors_list("::-webkit-calendar-picker-indicator")
        .declarations_block(CssLineHeight::new().content("1"))
        .into()
}

fn moz_ui_invalid_box_shadow() -> CssStatement {
    CssRule::new()
        .selectors_list(":-moz-ui-invalid")
        .declarations_block(CssBoxShadow::from(None))
        .into()
}

fn button_appearance() -> CssStatement {
    let selectors_list = [
        "button",
        "input:where([type='button'], [type='reset'], [type='submit'])",
        "::file-selector-button",
    ];
    let declarations_block = CssAppearance::from(Button);

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
        .into()
}

fn hidden_display() -> CssStatement {
    CssRule::new()
        .selectors_list("[hidden]:where(:not([hidden='until-found']))")
        .declarations_block(CssDisplay::from((None, Important)))
        .into()
}
