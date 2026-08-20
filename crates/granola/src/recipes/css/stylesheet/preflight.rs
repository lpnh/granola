//! Based on `preflight.css` by Tailwind Labs, Inc.
//! Source: https://github.com/tailwindlabs/tailwindcss/blob/main/packages/tailwindcss/preflight.css
//! Licensed under MIT License (https://github.com/tailwindlabs/tailwindcss/blob/main/LICENSE)

use crate::{macros::*, prelude::*, recipes::*};

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
    fn content_recipe() -> Bake {
        bake_ws![
            CssRule::from(UniversalReset),
            rule!(
                bake_comma!["html", ":host"],
                declarations_block![
                    CssDeclaration::from(LineHeight).value("1.5"),
                    CssDeclaration::from(WebkitTextSizeAdjust).value("100%"),
                    CssDeclaration::from(TabSize).value("4"),
                    CssDeclaration::from(FontFamily)
                        .value(CssCustomFunction::from(DefaultFontFamily)),
                    CssDeclaration::from(FontFeatureSettings)
                        .value(CssCustomFunction::from(DefaultFontFeatureSettings)),
                    CssDeclaration::from(FontVariationSettings)
                        .value(CssCustomFunction::from(DefaultFontVariationSettings)),
                    ("-webkit-tap-highlight-color", "transparent"),
                ]
            ),
            CssRule::from(HrReset),
            rule!(
                "abbr:where([title])",
                declarations_block![
                    CssDeclaration::from(WebkitTextDecoration).value("underline dotted"),
                    CssDeclaration::from(TextDecoration).value("underline dotted"),
                ]
            ),
            CssRule::from(AllHeadingsFontReset),
            CssRule::from(AnchorInherit),
            CssRule::from(BStrongFontWeight),
            CssRule::from(MonospaceSelectors).content(declarations_block![
                CssDeclaration::from(FontFamily)
                    .value(CssCustomFunction::from(DefaultMonoFontFamily)),
                CssDeclaration::from(FontFeatureSettings)
                    .value(CssCustomFunction::from(DefaultMonoFontFeatureSettings)),
                CssDeclaration::from(FontVariationSettings)
                    .value(CssCustomFunction::from(DefaultMonoFontVariationSettings)),
                CssDeclaration::from(FontSize).value("1em"),
            ]),
            CssRule::from(SmallFontSize),
            CssRule::from(SubSupDefaults),
            CssRule::from(SubVerticalPos),
            CssRule::from(SupVerticalPos),
            rule!(
                "table",
                declarations_block![
                    CssDeclaration::from(TextIndent).value("0"),
                    CssDeclaration::from(BorderColor).value("inherit"),
                    CssDeclaration::from(BorderCollapse).value("collapse"),
                ]
            ),
            rule!(
                ":-moz-focusring",
                CssDeclaration::from(Outline).value("auto")
            ),
            CssRule::from(ProgressVerticalAlignment),
            CssRule::from(SummaryDisplayListItem),
            rule!(
                bake_comma!["ol", "ul", "menu"],
                CssDeclaration::from(ListStyle).value("none")
            ),
            rule!(
                bake_comma![
                    "img", "svg", "video", "canvas", "audio", "iframe", "embed", "object",
                ],
                declarations_block![
                    CssDeclaration::from(Display).value("block"),
                    CssDeclaration::from(VerticalAlign).value("middle")
                ]
            ),
            rule!(
                bake_comma!["img", "video"],
                declarations_block![
                    CssDeclaration::from(MaxWidth).value("100%"),
                    CssDeclaration::from(Height).value("auto")
                ]
            ),
            CssRule::from(FormControlsExt)
                .content(declarations_block![
                    CssDeclaration::from(Font).value("inherit"),
                    CssDeclaration::from(FontFeatureSettings).value("inherit"),
                    CssDeclaration::from(FontVariationSettings).value("inherit"),
                    CssDeclaration::from(LetterSpacing).value("inherit"),
                    CssDeclaration::from(Color).value("inherit"),
                    CssDeclaration::from(BorderRadius).value("0"),
                    CssDeclaration::from(BackgroundColor).value("transparent"),
                    CssDeclaration::from(Opacity).value("1"),
                ])
                .push_selector(UniversalFileSelectorButton),
            rule!(
                CssSimpleSelector::new()
                    .selector(":where(select:is([multiple], [size]))")
                    .descendant("optgroup"),
                CssDeclaration::from(FontWeight).value("bolder")
            ),
            rule!(
                CssSimpleSelector::new()
                    .selector(":where(select:is([multiple], [size]))")
                    .descendant("optgroup")
                    .descendant("option"),
                CssDeclaration::from(PaddingInlineStart).value("20px")
            ),
            CssRule::new()
                .push_selector(UniversalFileSelectorButton)
                .push_property(CssDeclaration::from(MarginInlineEnd).value("4px")),
            CssRule::new()
                .push_selector(UniversalPlaceholder)
                .push_property(CssDeclaration::from(Opacity).value("1")),
            CssAtRule::new()
                .identifier("supports")
                .rule(
                    "(not (-webkit-appearance: -apple-pay-button)) or (contain-intrinsic-size: 1px)"
                )
                .block(
                    CssRule::new()
                        .push_selector(UniversalPlaceholder)
                        .push_property(
                            CssDeclaration::from(Color)
                                .value("color-mix(in oklab, currentcolor 50%, transparent)",)
                        ),
                ),
            rule!("textarea", CssDeclaration::from(Resize).value("vertical")),
            CssRule::from(SearchDecorationAppearance),
            rule!(
                "::-webkit-date-and-time-value",
                declarations_block![
                    CssDeclaration::from(MinHeight).value("1lh"),
                    CssDeclaration::from(TextAlign).value("inherit")
                ]
            ),
            rule!(
                "::-webkit-datetime-edit",
                CssDeclaration::from(Display).value("inline flex")
            ),
            rule!(
                "::-webkit-datetime-edit-fields-wrapper",
                CssDeclaration::from(Padding).value("0")
            ),
            rule!(
                bake_comma![
                    "::-webkit-datetime-edit",
                    "::-webkit-datetime-edit-year-field",
                    "::-webkit-datetime-edit-month-field",
                    "::-webkit-datetime-edit-day-field",
                    "::-webkit-datetime-edit-hour-field",
                    "::-webkit-datetime-edit-minute-field",
                    "::-webkit-datetime-edit-second-field",
                    "::-webkit-datetime-edit-millisecond-field",
                    "::-webkit-datetime-edit-meridiem-field",
                ],
                CssDeclaration::from(PaddingBlock).value("0")
            ),
            rule!(
                "::-webkit-calendar-picker-indicator",
                CssDeclaration::from(LineHeight).value("1")
            ),
            rule!(
                ":-moz-ui-invalid",
                CssDeclaration::from(BoxShadow).value("none")
            ),
            rule!(
                bake_comma![
                    "button",
                    "input:where([type='button'], [type='reset'], [type='submit'])",
                    "::file-selector-button",
                ],
                CssDeclaration::from(Appearance).value("button")
            ),
            CssRule::from(SpinButtonHeight),
            rule!(
                "[hidden]:where(:not([hidden='until-found']))",
                CssDeclaration::from(Display).value("none !important")
            ),
        ]
    }
}

/// The recipe for the default sans-serif font family.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(DefaultFontFamily);
///
/// assert_eq!(custom_property.bake(), "--default-font-family");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultFontFamily;

impl CustomPropertyRecipe for DefaultFontFamily {
    fn name_recipe() -> Bake {
        "default-font-family".into()
    }
}

impl CustomFunctionRecipe for DefaultFontFamily {
    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Bake {
        bake_comma![
            CssCustomProperty::from(Self),
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
/// let custom_property = CssCustomProperty::from(DefaultFontFeatureSettings);
///
/// assert_eq!(custom_property.bake(), "--default-font-feature-settings");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultFontFeatureSettings;

impl CustomPropertyRecipe for DefaultFontFeatureSettings {
    fn name_recipe() -> Bake {
        "default-font-feature-settings".into()
    }
}

impl CustomFunctionRecipe for DefaultFontFeatureSettings {
    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Bake {
        bake_comma![CssCustomProperty::from(Self), "normal"]
    }
}

/// The custom property for the default font variation settings.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(DefaultFontVariationSettings);
///
/// assert_eq!(custom_property.bake(), "--default-font-variation-settings");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultFontVariationSettings;

impl CustomPropertyRecipe for DefaultFontVariationSettings {
    fn name_recipe() -> Bake {
        "default-font-variation-settings".into()
    }
}

impl CustomFunctionRecipe for DefaultFontVariationSettings {
    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Bake {
        bake_comma![CssCustomProperty::from(Self), "normal"]
    }
}

/// The custom property for the default monospace font family.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(DefaultMonoFontFamily);
///
/// assert_eq!(custom_property.bake(), "--default-mono-font-family");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultMonoFontFamily;

impl CustomPropertyRecipe for DefaultMonoFontFamily {
    fn name_recipe() -> Bake {
        "default-mono-font-family".into()
    }
}

impl CustomFunctionRecipe for DefaultMonoFontFamily {
    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Bake {
        bake_comma![
            CssCustomProperty::from(Self),
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
/// let custom_property = CssCustomProperty::from(DefaultMonoFontFeatureSettings);
///
/// assert_eq!(
///     custom_property.bake(),
///     "--default-mono-font-feature-settings"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultMonoFontFeatureSettings;

impl CustomPropertyRecipe for DefaultMonoFontFeatureSettings {
    fn name_recipe() -> Bake {
        "default-mono-font-feature-settings".into()
    }
}

impl CustomFunctionRecipe for DefaultMonoFontFeatureSettings {
    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Bake {
        bake_comma![CssCustomProperty::from(Self), "normal"]
    }
}

/// The custom property for the default monospace font variation settings.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_property = CssCustomProperty::from(DefaultMonoFontVariationSettings);
///
/// assert_eq!(
///     custom_property.bake(),
///     "--default-mono-font-variation-settings"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DefaultMonoFontVariationSettings;

impl CustomPropertyRecipe for DefaultMonoFontVariationSettings {
    fn name_recipe() -> Bake {
        "default-mono-font-variation-settings".into()
    }
}

impl CustomFunctionRecipe for DefaultMonoFontVariationSettings {
    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Bake {
        bake_comma![CssCustomProperty::from(Self), "normal"]
    }
}

/// The CSS `--theme(...)` custom function.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let custom_function = CssCustomFunction::from(Theme);
///
/// assert_eq!(custom_function.bake(), "--theme()");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Theme;

impl CustomFunctionRecipe for Theme {
    fn name_recipe() -> Bake {
        "theme".into()
    }
}
