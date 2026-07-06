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
    fn statements_recipe() -> Bake {
        bake_ws![
            CssRule::from(UniversalReset),
            html_host_defaults(),
            CssRule::from(HrReset),
            abbr_text_decoration(),
            CssRule::from(AllHeadingsFontReset),
            CssRule::from(AnchorInherit),
            CssRule::from(BStrongFontWeight),
            monospace_defaults(),
            CssRule::from(SmallFontSize),
            CssRule::from(SubSupDefaults),
            CssRule::from(SubVerticalPos),
            CssRule::from(SupVerticalPos),
            table_reset(),
            moz_focusring_outline(),
            CssRule::from(ProgressVerticalAlignment),
            CssRule::from(SummaryDisplayListItem),
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
            CssRule::from(SearchDecorationAppearance),
            date_and_time_value(),
            datetime_edit_display(),
            datetime_edit_fields_wrapper(),
            datetime_edit_padding_block(),
            calendar_picker_indicator(),
            moz_ui_invalid_box_shadow(),
            button_appearance(),
            CssRule::from(SpinButtonHeight),
            hidden_display(),
        ]
    }
}

fn html_host_defaults() -> CssRule {
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

    let selectors_list = bake_comma!["html", ":host"];
    let declarations_block = bake_ws![
        CssDeclaration::from(LineHeight).content("1.5"),
        CssDeclaration::from(WebkitTextSizeAdjust).content("100%"),
        CssDeclaration::from(TabSize).content("4"),
        CssDeclaration::from(FontFamily).content(sans_font_family),
        CssDeclaration::from(FontFeatureSettings)
            .content("--theme(--default-font-feature-settings, normal)"),
        CssDeclaration::from(FontVariationSettings)
            .content("--theme(--default-font-variation-settings, normal)"),
        CssDeclaration::from(("-webkit-tap-highlight-color", "transparent")),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
}

fn abbr_text_decoration() -> CssRule {
    let selectors_list = "abbr:where([title])";
    let declarations_block = bake_ws![
        CssDeclaration::from(WebkitTextDecoration).content(bake_ws!["underline", "dotted"]),
        CssDeclaration::from(TextDecoration).content(bake_ws!["underline", "dotted"]),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
}

fn monospace_defaults() -> CssRule<MonospaceSelectors> {
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

    let declarations_block = bake_ws![
        CssDeclaration::from(FontFamily).content(mono_font_family),
        CssDeclaration::from(FontFeatureSettings)
            .content("--theme(--default-mono-font-feature-settings, normal)"),
        CssDeclaration::from(FontVariationSettings)
            .content("--theme(--default-mono-font-variation-settings, normal)"),
        CssDeclaration::from(FontSize).content("1em"),
    ];

    CssRule::from(MonospaceSelectors).declarations_block(declarations_block)
}

fn table_reset() -> CssRule {
    let selectors_list = "table";
    let declarations_block = bake_ws![
        CssDeclaration::from(TextIndent).content("0"),
        CssDeclaration::from(BorderColor).inherit(),
        CssDeclaration::from(BorderCollapse).content("collapse"),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
}

fn moz_focusring_outline() -> CssRule {
    CssRule::new()
        .selectors_list(":-moz-focusring")
        .declarations_block(CssDeclaration::from(Outline).content("auto"))
}

fn list_reset() -> CssRule {
    CssRule::new()
        .selectors_list(bake_comma!["ol", "ul", "menu"])
        .declarations_block(CssDeclaration::from(ListStyle).content("none"))
}

fn replaced_element_display() -> CssRule {
    let selectors_list = bake_comma![
        "img", "svg", "video", "canvas", "audio", "iframe", "embed", "object",
    ];
    let declarations_block = bake_ws![
        CssDeclaration::from(Display).content("block"),
        CssDeclaration::from(VerticalAlign).content("middle")
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
}

fn replaced_element_sizing() -> CssRule {
    CssRule::new()
        .selectors_list(bake_comma!["img", "video"])
        .declarations_block(bake_ws![
            CssDeclaration::from(MaxWidth).content("100%"),
            CssDeclaration::from(Height).content("auto")
        ])
}

fn form_controls_reset() -> CssRule<FormControlsExt> {
    let declarations_block = bake_ws![
        CssDeclaration::from(Font).inherit(),
        CssDeclaration::from(FontFeatureSettings).inherit(),
        CssDeclaration::from(FontVariationSettings).inherit(),
        CssDeclaration::from(LetterSpacing).inherit(),
        CssDeclaration::from(Color).inherit(),
        CssDeclaration::from(BorderRadius).content("0"),
        CssDeclaration::from(BackgroundColor).content("transparent"),
        CssDeclaration::from(Opacity).content("1"),
    ];

    CssRule::from(FormControlsExt)
        .push_selector(CssSimpleSelector::from(UniversalFileSelectorButton))
        .declarations_block(declarations_block)
}

fn optgroup_font_weight() -> CssRule {
    let selectors_list = CssSimpleSelector::new()
        .selector(":where(select:is([multiple], [size]))")
        .descendant("optgroup");
    let declarations_block = CssDeclaration::from(FontWeight)
        .content("bolder")
        .bake_recipe();

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
}

fn optgroup_option_indent() -> CssRule {
    let selectors_list = CssSimpleSelector::new()
        .selector(":where(select:is([multiple], [size]))")
        .descendant("optgroup")
        .descendant("option");
    let declarations_block = CssDeclaration::from(PaddingInlineStart).content("20px");

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
}

fn file_selector_button_spacing() -> CssRule {
    CssRule::new()
        .selectors_list(CssSimpleSelector::from(UniversalFileSelectorButton))
        .declarations_block(CssDeclaration::from(MarginInlineEnd).content("4px"))
}

fn placeholder_opacity() -> CssRule {
    CssRule::new()
        .selectors_list(CssSimpleSelector::from(UniversalPlaceholder))
        .declarations_block(CssDeclaration::from(Opacity).content("1"))
}

fn placeholder_color_supports() -> CssAtRule {
    let condition = "(not (-webkit-appearance: -apple-pay-button)) or
  (contain-intrinsic-size: 1px)";

    let placeholder = CssRule::new()
        .selectors_list(CssSimpleSelector::from(UniversalPlaceholder))
        .declarations_block(
            CssDeclaration::from(Color)
                .content("color-mix(in oklab, currentcolor 50%, transparent)"),
        );

    CssAtRule::new()
        .identifier("supports")
        .rule(condition)
        .block(placeholder)
}

fn textarea_resize() -> CssRule {
    CssRule::new()
        .selectors_list("textarea")
        .declarations_block(CssDeclaration::from(Resize).content("vertical"))
}

fn date_and_time_value() -> CssRule {
    let selectors_list = "::-webkit-date-and-time-value";
    let declarations_block = bake_ws![
        CssDeclaration::from(MinHeight).content("1lh"),
        CssDeclaration::from(TextAlign).inherit(),
    ];

    CssRule::new()
        .selectors_list(selectors_list)
        .declarations_block(declarations_block)
}

fn datetime_edit_display() -> CssRule {
    CssRule::new()
        .selectors_list("::-webkit-datetime-edit")
        .declarations_block(CssDeclaration::from(Display).content("inline flex"))
}

fn datetime_edit_fields_wrapper() -> CssRule {
    CssRule::new()
        .selectors_list("::-webkit-datetime-edit-fields-wrapper")
        .declarations_block(CssDeclaration::from(Padding).content("0"))
}

fn datetime_edit_padding_block() -> CssRule {
    CssRule::new()
        .selectors_list(bake_comma![
            "::-webkit-datetime-edit",
            "::-webkit-datetime-edit-year-field",
            "::-webkit-datetime-edit-month-field",
            "::-webkit-datetime-edit-day-field",
            "::-webkit-datetime-edit-hour-field",
            "::-webkit-datetime-edit-minute-field",
            "::-webkit-datetime-edit-second-field",
            "::-webkit-datetime-edit-millisecond-field",
            "::-webkit-datetime-edit-meridiem-field",
        ])
        .declarations_block(CssDeclaration::from(PaddingBlock).content("0"))
}

fn calendar_picker_indicator() -> CssRule {
    CssRule::new()
        .selectors_list("::-webkit-calendar-picker-indicator")
        .declarations_block(CssDeclaration::from(LineHeight).content("1"))
}

fn moz_ui_invalid_box_shadow() -> CssRule {
    CssRule::new()
        .selectors_list(":-moz-ui-invalid")
        .declarations_block(CssDeclaration::from(BoxShadow).content("none"))
}

fn button_appearance() -> CssRule {
    CssRule::new()
        .selectors_list(bake_comma![
            "button",
            "input:where([type='button'], [type='reset'], [type='submit'])",
            "::file-selector-button",
        ])
        .declarations_block(CssDeclaration::from(Appearance).content("button"))
}

fn hidden_display() -> CssRule {
    CssRule::new()
        .selectors_list("[hidden]:where(:not([hidden='until-found']))")
        .declarations_block(CssDeclaration::from(Display).content("none").important())
}

/// The recipe for the default sans-serif font family.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(DefaultFontFamily);
///
/// assert_eq!(
///     custom_property.bake(),
///     "--default-font-family: ui-sans-serif, system-ui, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji';"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultFontFamily;

impl CustomPropertyRecipe for DefaultFontFamily {
    fn name_recipe() -> Bake {
        "default-font-family".into()
    }
}

impl DeclarationRecipe for DefaultFontFamily {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        bake_comma![
            "ui-sans-serif",
            "system-ui",
            "sans-serif",
            "'Apple Color Emoji'",
            "'Segoe UI Emoji'",
            "'Segoe UI Symbol'",
            "'Noto Color Emoji'",
        ]
    }
}

/// The custom property for the default font feature settings.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(DefaultFontFeatureSettings);
///
/// assert_eq!(
///     custom_property.bake(),
///     "--default-font-feature-settings: normal;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultFontFeatureSettings;

impl CustomPropertyRecipe for DefaultFontFeatureSettings {
    fn name_recipe() -> Bake {
        "default-font-feature-settings".into()
    }
}

impl DeclarationRecipe for DefaultFontFeatureSettings {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "normal".into()
    }
}

/// The custom property for the default font variation settings.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(DefaultFontVariationSettings);
///
/// assert_eq!(
///     custom_property.bake(),
///     "--default-font-variation-settings: normal;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultFontVariationSettings;

impl CustomPropertyRecipe for DefaultFontVariationSettings {
    fn name_recipe() -> Bake {
        "default-font-variation-settings".into()
    }
}

impl DeclarationRecipe for DefaultFontVariationSettings {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "normal".into()
    }
}

/// The custom property for the default monospace font family.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(DefaultMonoFontFamily);
///
/// assert_eq!(
///     custom_property.bake(),
///     "--default-mono-font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultMonoFontFamily;

impl CustomPropertyRecipe for DefaultMonoFontFamily {
    fn name_recipe() -> Bake {
        "default-mono-font-family".into()
    }
}

impl DeclarationRecipe for DefaultMonoFontFamily {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        bake_comma![
            "ui-monospace",
            "SFMono-Regular",
            "Menlo",
            "Monaco",
            "Consolas",
            "'Liberation Mono'",
            "'Courier New'",
            "monospace",
        ]
    }
}

/// The custom property for the default monospace font feature settings.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(DefaultMonoFontFeatureSettings);
///
/// assert_eq!(
///     custom_property.bake(),
///     "--default-mono-font-feature-settings: normal;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultMonoFontFeatureSettings;

impl CustomPropertyRecipe for DefaultMonoFontFeatureSettings {
    fn name_recipe() -> Bake {
        "default-mono-font-feature-settings".into()
    }
}

impl DeclarationRecipe for DefaultMonoFontFeatureSettings {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "normal".into()
    }
}

/// The custom property for the default monospace font variation settings.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssDeclaration::from(DefaultMonoFontVariationSettings);
///
/// assert_eq!(
///     custom_property.bake(),
///     "--default-mono-font-variation-settings: normal;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultMonoFontVariationSettings;

impl CustomPropertyRecipe for DefaultMonoFontVariationSettings {
    fn name_recipe() -> Bake {
        "default-mono-font-variation-settings".into()
    }
}

impl DeclarationRecipe for DefaultMonoFontVariationSettings {
    recipe_boilerplate!(DeclarationRecipe);

    fn property_recipe() -> Bake {
        CssCustomProperty::from(Self).into()
    }

    fn content_recipe() -> Self::Content {
        "normal".into()
    }
}
