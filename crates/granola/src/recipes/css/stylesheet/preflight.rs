// Based on `preflight.css` by Tailwind Labs, Inc.
// Source: https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/preflight.css
// Licensed under MIT License (https://github.com/tailwindlabs/tailwindcss/blob/main/LICENSE)

use crate::{recipes::*, prelude::*};

/// The preflight stylesheet recipe.
///
/// [preflight source code](https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/preflight.css)
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let stylesheet: CssStylesheet<Preflight> = CssStylesheet::from_cookbook();
///
/// assert_eq!(
///     stylesheet.bake(),
///     r#"*,
/// ::after,
/// ::before,
/// ::backdrop,
/// ::file-selector-button {
///   box-sizing: border-box;
///   margin: 0;
///   padding: 0;
///   border: 0 solid;
/// }
///
/// html,
/// :host {
///   line-height: 1.5;
///   -webkit-text-size-adjust: 100%;
///   tab-size: 4;
///   font-family: --theme(
///     --default-font-family,
///     ui-sans-serif,
///     system-ui,
///     sans-serif,
///     'Apple Color Emoji',
///     'Segoe UI Emoji',
///     'Segoe UI Symbol',
///     'Noto Color Emoji'
///   );
///   font-feature-settings: --theme(--default-font-feature-settings, normal);
///   font-variation-settings: --theme(--default-font-variation-settings, normal);
///   -webkit-tap-highlight-color: transparent;
/// }
///
/// hr {
///   height: 0;
///   color: inherit;
///   border-top-width: 1px;
/// }
///
/// abbr:where([title]) {
///   -webkit-text-decoration: underline dotted;
///   text-decoration: underline dotted;
/// }
///
/// h1,
/// h2,
/// h3,
/// h4,
/// h5,
/// h6 {
///   font-size: inherit;
///   font-weight: inherit;
/// }
///
/// a {
///   color: inherit;
///   -webkit-text-decoration: inherit;
///   text-decoration: inherit;
/// }
///
/// b,
/// strong {
///   font-weight: bolder;
/// }
///
/// code,
/// kbd,
/// samp,
/// pre {
///   font-family: --theme(
///     --default-mono-font-family,
///     ui-monospace,
///     SFMono-Regular,
///     Menlo,
///     Monaco,
///     Consolas,
///     'Liberation Mono',
///     'Courier New',
///     monospace
///   );
///   font-feature-settings: --theme(--default-mono-font-feature-settings, normal);
///   font-variation-settings: --theme(--default-mono-font-variation-settings, normal);
///   font-size: 1em;
/// }
///
/// small {
///   font-size: 80%;
/// }
///
/// sub,
/// sup {
///   font-size: 75%;
///   line-height: 0;
///   position: relative;
///   vertical-align: baseline;
/// }
///
/// sub {
///   bottom: -0.25em;
/// }
///
/// sup {
///   top: -0.5em;
/// }
///
/// table {
///   text-indent: 0;
///   border-color: inherit;
///   border-collapse: collapse;
/// }
///
/// :-moz-focusring {
///   outline: auto;
/// }
///
/// progress {
///   vertical-align: baseline;
/// }
///
/// summary {
///   display: list-item;
/// }
///
/// ol,
/// ul,
/// menu {
///   list-style: none;
/// }
///
/// img,
/// svg,
/// video,
/// canvas,
/// audio,
/// iframe,
/// embed,
/// object {
///   display: block;
///   vertical-align: middle;
/// }
///
/// img,
/// video {
///   max-width: 100%;
///   height: auto;
/// }
///
/// button,
/// input,
/// optgroup,
/// select,
/// textarea,
/// ::file-selector-button {
///   font: inherit;
///   font-feature-settings: inherit;
///   font-variation-settings: inherit;
///   letter-spacing: inherit;
///   color: inherit;
///   border-radius: 0;
///   background-color: transparent;
///   opacity: 1;
/// }
///
/// :where(select:is([multiple], [size])) optgroup {
///   font-weight: bolder;
/// }
///
/// :where(select:is([multiple], [size])) optgroup option {
///   padding-inline-start: 20px;
/// }
///
/// ::file-selector-button {
///   margin-inline-end: 4px;
/// }
///
/// ::placeholder {
///   opacity: 1;
/// }
///
/// @supports (not (-webkit-appearance: -apple-pay-button)) or
///   (contain-intrinsic-size: 1px) {
///   ::placeholder {
///     color: color-mix(in oklab, currentcolor 50%, transparent);
///   }
/// }
///
/// textarea {
///   resize: vertical;
/// }
///
/// ::-webkit-search-decoration {
///   -webkit-appearance: none;
/// }
///
/// ::-webkit-date-and-time-value {
///   min-height: 1lh;
///   text-align: inherit;
/// }
///
/// ::-webkit-datetime-edit {
///   display: inline flex;
/// }
///
/// ::-webkit-datetime-edit-fields-wrapper {
///   padding: 0;
/// }
///
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
///
/// ::-webkit-calendar-picker-indicator {
///   line-height: 1;
/// }
///
/// :-moz-ui-invalid {
///   box-shadow: none;
/// }
///
/// button,
/// input:where([type='button'], [type='reset'], [type='submit']),
/// ::file-selector-button {
///   appearance: button;
/// }
///
/// ::-webkit-inner-spin-button,
/// ::-webkit-outer-spin-button {
///   height: auto;
/// }
///
/// [hidden]:where(:not([hidden='until-found'])) {
///   display: none !important;
/// }"#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Preflight;

impl StylesheetRecipe for Preflight {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([
            CssRule::<UniversalReset>::from_cookbook().into(),
            html_host_defaults(),
            CssRule::<HrReset>::from_cookbook().into(),
            abbr_text_decoration(),
            CssRule::<AllHeadingsFontReset>::from_cookbook().into(),
            CssRule::<AnchorInherit>::from_cookbook().into(),
            CssRule::<BStrongFontWeight>::from_cookbook().into(),
            monospace_defaults(),
            CssRule::<SmallFontSize>::from_cookbook().into(),
            CssRule::<SubSupDefaults>::from_cookbook().into(),
            CssRule::<SubVerticalPos>::from_cookbook().into(),
            CssRule::<SupVerticalPos>::from_cookbook().into(),
            table_reset(),
            moz_focusring_outline(),
            CssRule::<ProgressVerticalAlignment>::from_cookbook().into(),
            CssRule::<SummaryDisplay>::from_cookbook().into(),
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
            CssRule::<SearchDecorationAppearance>::from_cookbook().into(),
            date_and_time_value(),
            datetime_edit_display(),
            datetime_edit_fields_wrapper(),
            datetime_edit_padding_block(),
            calendar_picker_indicator(),
            moz_ui_invalid_box_shadow(),
            button_appearance(),
            CssRule::<SpinButtonHeight>::from_cookbook().into(),
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

    let selectors_list: CssSelectorsList = ["html", ":host"].into();
    let declarations_block: [CssDeclaration; 7] = [
        CssLineHeight::<()>::new("1.5").into(),
        CssWebkitTextSizeAdjust::<()>::new("100%").into(),
        CssTabSize::<()>::new("4").into(),
        CssFontFamily::<()>::new(sans_font_family).into(),
        CssFontFeatureSettings::<()>::new("--theme(--default-font-feature-settings, normal)")
            .into(),
        CssFontVariationSettings::<()>::new("--theme(--default-font-variation-settings, normal)")
            .into(),
        ("-webkit-tap-highlight-color", "transparent").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn abbr_text_decoration() -> CssStatement {
    let selectors_list: CssSelectorsList = "abbr:where([title])".into();
    let declarations_block: [CssDeclaration; 2] = [
        CssWebkitTextDecoration::<(Underline, Dotted)>::from_cookbook().into(),
        CssTextDecoration::<(Underline, Dotted)>::from_cookbook().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
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

    let selectors_list = CssSelectorsList::<MonospaceSelectors>::from_cookbook().bake_recipe();
    let declarations_block: [CssDeclaration; 4] = [
        CssFontFamily::<()>::new(mono_font_family).into(),
        CssFontFeatureSettings::<()>::new("--theme(--default-mono-font-feature-settings, normal)")
            .into(),
        CssFontVariationSettings::<()>::new(
            "--theme(--default-mono-font-variation-settings, normal)",
        )
        .into(),
        CssFontSize::<()>::new("1em").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn table_reset() -> CssStatement {
    let selectors_list: CssSelectorsList = "table".into();
    let declarations_block: [CssDeclaration; 3] = [
        CssTextIndent::<()>::new("0").into(),
        CssBorderColor::<Inherit>::from_cookbook().into(),
        CssBorderCollapse::<Collapse>::from_cookbook().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn moz_focusring_outline() -> CssStatement {
    let selectors_list: CssSelectorsList = ":-moz-focusring".into();
    let declarations_block: CssDeclaration = CssOutline::<Auto>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn list_reset() -> CssStatement {
    let selectors_list: CssSelectorsList = ["ol", "ul", "menu"].into();
    let declarations_block: CssDeclaration = CssListStyle::<None>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn replaced_element_display() -> CssStatement {
    let selectors_list: CssSelectorsList = [
        "img", "svg", "video", "canvas", "audio", "iframe", "embed", "object",
    ]
    .into();
    let declarations_block: [CssDeclaration; 2] = [
        CssDisplay::<Block>::from_cookbook().into(),
        CssVerticalAlign::<Middle>::from_cookbook().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn replaced_element_sizing() -> CssStatement {
    let selectors_list: CssSelectorsList = ["img", "video"].into();
    let declarations_block: [CssDeclaration; 2] = [
        CssMaxWidth::<()>::new("100%").into(),
        CssHeight::<Auto>::from_cookbook().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn form_controls_reset() -> CssStatement {
    let selectors_list = CssSelectorsList::<FormControlsExt>::from_cookbook()
        .push(CssSimpleSelector::<UniversalFileSelectorButton>::from_cookbook())
        .bake_recipe();
    let declarations_block: [CssDeclaration; 8] = [
        CssFont::<Inherit>::from_cookbook().into(),
        CssFontFeatureSettings::<Inherit>::from_cookbook().into(),
        CssFontVariationSettings::<Inherit>::from_cookbook().into(),
        CssLetterSpacing::<Inherit>::from_cookbook().into(),
        CssColor::<Inherit>::from_cookbook().into(),
        CssBorderRadius::<()>::new("0").into(),
        CssBackgroundColor::<Transparent>::from_cookbook().into(),
        CssOpacity::<()>::new("1").into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn optgroup_font_weight() -> CssStatement {
    let selectors_list: CssSelectorsList =
        CssSimpleSelector::<()>::new(":where(select:is([multiple], [size]))")
            .descendant("optgroup")
            .into();
    let declarations_block: CssDeclaration = CssFontWeight::<Bolder>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn optgroup_option_indent() -> CssStatement {
    let selectors_list: CssSelectorsList =
        CssSimpleSelector::<()>::new(":where(select:is([multiple], [size]))")
            .descendant("optgroup")
            .descendant("option")
            .into();
    let declarations_block: CssDeclaration = CssPaddingInlineStart::<()>::new("20px").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn file_selector_button_spacing() -> CssStatement {
    let selectors_list = CssSimpleSelector::<UniversalFileSelectorButton>::from_cookbook();
    let declarations_block: CssDeclaration = CssMarginInlineEnd::<()>::new("4px").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn placeholder_opacity() -> CssStatement {
    let selectors_list = CssSimpleSelector::<UniversalPlaceholder>::from_cookbook();
    let declarations_block: CssDeclaration = CssOpacity::<()>::new("1").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn placeholder_color_supports() -> CssStatement {
    let condition = "(not (-webkit-appearance: -apple-pay-button)) or
  (contain-intrinsic-size: 1px)";

    let placeholder: CssRule = CssRule::<()>::new(
        CssSimpleSelector::<UniversalPlaceholder>::from_cookbook(),
        CssColor::<()>::new("color-mix(in oklab, currentcolor 50%, transparent)"),
    );

    CssAtRule::<()>::new("supports", condition)
        .block(placeholder)
        .into()
}

fn textarea_resize() -> CssStatement {
    let selectors_list: CssSelectorsList = "textarea".into();
    let declarations_block: CssDeclaration = CssResize::<Vertical>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn date_and_time_value() -> CssStatement {
    let selectors_list: CssSelectorsList = "::-webkit-date-and-time-value".into();
    let declarations_block: [CssDeclaration; 2] = [
        CssMinHeight::<()>::new("1lh").into(),
        CssTextAlign::<Inherit>::from_cookbook().into(),
    ];

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn datetime_edit_display() -> CssStatement {
    let selectors_list: CssSelectorsList = "::-webkit-datetime-edit".into();
    let declarations_block: CssDeclaration = CssDisplay::<InlineFlex>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn datetime_edit_fields_wrapper() -> CssStatement {
    let selectors_list: CssSelectorsList = "::-webkit-datetime-edit-fields-wrapper".into();
    let declarations_block: CssDeclaration = CssPadding::<()>::new("0").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn datetime_edit_padding_block() -> CssStatement {
    let selectors_list: CssSelectorsList = [
        "::-webkit-datetime-edit",
        "::-webkit-datetime-edit-year-field",
        "::-webkit-datetime-edit-month-field",
        "::-webkit-datetime-edit-day-field",
        "::-webkit-datetime-edit-hour-field",
        "::-webkit-datetime-edit-minute-field",
        "::-webkit-datetime-edit-second-field",
        "::-webkit-datetime-edit-millisecond-field",
        "::-webkit-datetime-edit-meridiem-field",
    ]
    .into();
    let declarations_block: CssDeclaration = CssPaddingBlock::<()>::new("0").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn calendar_picker_indicator() -> CssStatement {
    let selectors_list: CssSelectorsList = "::-webkit-calendar-picker-indicator".into();
    let declarations_block: CssDeclaration = CssLineHeight::<()>::new("1").into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn moz_ui_invalid_box_shadow() -> CssStatement {
    let selectors_list: CssSelectorsList = ":-moz-ui-invalid".into();
    let declarations_block: CssDeclaration = CssBoxShadow::<None>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn button_appearance() -> CssStatement {
    let selectors_list: CssSelectorsList = [
        "button",
        "input:where([type='button'], [type='reset'], [type='submit'])",
        "::file-selector-button",
    ]
    .into();
    let declarations_block: CssDeclaration = CssAppearance::<Button>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}

fn hidden_display() -> CssStatement {
    let selectors_list: CssSelectorsList = "[hidden]:where(:not([hidden='until-found']))".into();
    let declarations_block: CssDeclaration = CssDisplay::<(None, Important)>::from_cookbook().into();

    CssRule::<()>::new(selectors_list, declarations_block).into()
}
