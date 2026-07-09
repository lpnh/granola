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
    recipe_boilerplate!(StylesheetRecipe);

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssRule::from(UniversalReset),
            rule!(
                bake_comma!["html", ":host"],
                declarations_block![
                    (LineHeight, "1.5"),
                    (WebkitTextSizeAdjust, "100%"),
                    (TabSize, "4"),
                    (FontFamily, CssCustomFunction::from(DefaultFontFamily)),
                    (
                        FontFeatureSettings,
                        CssCustomFunction::from(DefaultFontFeatureSettings)
                    ),
                    (
                        FontVariationSettings,
                        CssCustomFunction::from(DefaultFontVariationSettings)
                    ),
                    ("-webkit-tap-highlight-color", "transparent"),
                ]
            ),
            CssRule::from(HrReset),
            rule!(
                "abbr:where([title])",
                declarations_block![
                    (WebkitTextDecoration, "underline dotted"),
                    (TextDecoration, "underline dotted"),
                ]
            ),
            CssRule::from(AllHeadingsFontReset),
            CssRule::from(AnchorInherit),
            CssRule::from(BStrongFontWeight),
            CssRule::from((
                MonospaceSelectors,
                declarations_block![
                    (FontFamily, CssCustomFunction::from(DefaultMonoFontFamily)),
                    (
                        FontFeatureSettings,
                        CssCustomFunction::from(DefaultMonoFontFeatureSettings)
                    ),
                    (
                        FontVariationSettings,
                        CssCustomFunction::from(DefaultMonoFontVariationSettings)
                    ),
                    (FontSize, "1em"),
                ]
            )),
            CssRule::from(SmallFontSize),
            CssRule::from(SubSupDefaults),
            CssRule::from(SubVerticalPos),
            CssRule::from(SupVerticalPos),
            rule!(
                "table",
                declarations_block![
                    (TextIndent, "0"),
                    (BorderColor, "inherit"),
                    (BorderCollapse, "collapse"),
                ]
            ),
            rule!(":-moz-focusring", declaration!(Outline, "auto")),
            CssRule::from(ProgressVerticalAlignment),
            CssRule::from(SummaryDisplayListItem),
            rule!(
                bake_comma!["ol", "ul", "menu"],
                declaration!(ListStyle, "none")
            ),
            rule!(
                bake_comma![
                    "img", "svg", "video", "canvas", "audio", "iframe", "embed", "object",
                ],
                declarations_block![(Display, "block"), (VerticalAlign, "middle")]
            ),
            rule!(
                bake_comma!["img", "video"],
                declarations_block![(MaxWidth, "100%"), (Height, "auto")]
            ),
            CssRule::from((
                FormControlsExt,
                declarations_block![
                    (Font, "inherit"),
                    (FontFeatureSettings, "inherit"),
                    (FontVariationSettings, "inherit"),
                    (LetterSpacing, "inherit"),
                    (Color, "inherit"),
                    (BorderRadius, "0"),
                    (BackgroundColor, "transparent"),
                    (Opacity, "1"),
                ]
            ))
            .push_selector(UniversalFileSelectorButton),
            rule!(
                CssSimpleSelector::new()
                    .selector(":where(select:is([multiple], [size]))")
                    .descendant("optgroup"),
                declaration!(FontWeight, "bolder")
            ),
            rule!(
                CssSimpleSelector::new()
                    .selector(":where(select:is([multiple], [size]))")
                    .descendant("optgroup")
                    .descendant("option"),
                declaration!(PaddingInlineStart, "20px")
            ),
            CssRule::new()
                .push_selector(UniversalFileSelectorButton)
                .push_property((MarginInlineEnd, "4px")),
            CssRule::new()
                .push_selector(UniversalPlaceholder)
                .push_property((Opacity, "1")),
            CssAtRule::new()
                .identifier("supports")
                .rule(
                    "(not (-webkit-appearance: -apple-pay-button)) or (contain-intrinsic-size: 1px)"
                )
                .block(
                    CssRule::new()
                        .push_selector(UniversalPlaceholder)
                        .push_property((
                            Color,
                            "color-mix(in oklab, currentcolor 50%, transparent)"
                        )),
                ),
            rule!("textarea", declaration!(Resize, "vertical")),
            CssRule::from(SearchDecorationAppearance),
            rule!(
                "::-webkit-date-and-time-value",
                declarations_block![(MinHeight, "1lh"), (TextAlign, "inherit")]
            ),
            rule!(
                "::-webkit-datetime-edit",
                declaration!(Display, "inline flex")
            ),
            rule!(
                "::-webkit-datetime-edit-fields-wrapper",
                declaration!(Padding, "0")
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
                declaration!(PaddingBlock, "0")
            ),
            rule!(
                "::-webkit-calendar-picker-indicator",
                declaration!(LineHeight, "1")
            ),
            rule!(":-moz-ui-invalid", declaration!(BoxShadow, "none")),
            rule!(
                bake_comma![
                    "button",
                    "input:where([type='button'], [type='reset'], [type='submit'])",
                    "::file-selector-button",
                ],
                declaration!(Appearance, "button")
            ),
            CssRule::from(SpinButtonHeight),
            rule!(
                "[hidden]:where(:not([hidden='until-found']))",
                declaration!(Display, "none !important")
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
    recipe_boilerplate!(DeclarationRecipe);

    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Self::Content {
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
    recipe_boilerplate!(DeclarationRecipe);

    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Self::Content {
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
    recipe_boilerplate!(DeclarationRecipe);

    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Self::Content {
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
    recipe_boilerplate!(DeclarationRecipe);

    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Self::Content {
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
    recipe_boilerplate!(DeclarationRecipe);

    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Self::Content {
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
    recipe_boilerplate!(DeclarationRecipe);

    fn name_recipe() -> Bake {
        Theme::name_recipe()
    }

    fn content_recipe() -> Self::Content {
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
    recipe_boilerplate!(CustomFunctionRecipe);

    fn name_recipe() -> Bake {
        "theme".into()
    }
}
