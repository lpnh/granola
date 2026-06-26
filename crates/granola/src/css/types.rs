use std::borrow::Cow;

/// Named color spaces.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Glossary/Color_space)
#[derive(strum::Display, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorSpace {
    /// The sRGB color space.
    #[strum(serialize = "srgb")]
    Srgb,
    /// The linear-light sRGB color space.
    #[strum(serialize = "srgb-linear")]
    SrgbLinear,
    /// The Display P3 color space.
    #[strum(serialize = "display-p3")]
    DisplayP3,
    /// The linear-light Display P3 color space.
    #[strum(serialize = "display-p3-linear")]
    DisplayP3Linear,
    /// The Adobe RGB (1998) color space.
    #[strum(serialize = "a98-rgb")]
    A98Rgb,
    /// The ProPhoto RGB color space.
    #[strum(serialize = "prophoto-rgb")]
    ProphotoRgb,
    /// The Rec. 2020 color space.
    #[strum(serialize = "rec2020")]
    Rec2020,
    /// The CIELAB color space.
    #[strum(serialize = "lab")]
    Lab,
    /// The Oklab color space.
    #[strum(serialize = "oklab")]
    Oklab,
    /// The CIE XYZ color space (D65 white point).
    #[strum(serialize = "xyz")]
    Xyz,
    /// The CIE XYZ color space with a D50 white point.
    #[strum(serialize = "xyz-d50")]
    XyzD50,
    /// The CIE XYZ color space with a D65 white point.
    #[strum(serialize = "xyz-d65")]
    XyzD65,
    /// The HSL (hue, saturation, lightness) polar color space.
    #[strum(serialize = "hsl")]
    Hsl,
    /// The HWB (hue, whiteness, blackness) polar color space.
    #[strum(serialize = "hwb")]
    Hwb,
    /// The CIELCH polar color space.
    #[strum(serialize = "lch")]
    Lch,
    /// The Oklch polar color space.
    #[strum(serialize = "oklch")]
    Oklch,
}

impl From<ColorSpace> for Cow<'static, str> {
    fn from(color_space: ColorSpace) -> Self {
        <&'static str>::from(color_space).into()
    }
}
