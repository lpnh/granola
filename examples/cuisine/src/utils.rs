use std::str::FromStr;

#[derive(Clone)]
pub struct Palette {
    pub source: String,
    pub base_100: String,
    pub base_200: String,
    pub base_300: String,
    pub base_content: String,
}

impl Palette {
    pub fn from_hex(hex: &str) -> Option<Self> {
        let base_srgb: sRGB = hex.parse().ok()?;
        let base_linear_rgb = LinearRGB::from(base_srgb);
        let base_oklab = Oklab::from(&base_linear_rgb);
        let base_oklch = Oklch::from(base_oklab);

        // luminance threshold where light and dark content give equal contrast
        // source: https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio
        let relative_luminance = Luminance::from(&base_linear_rgb);
        let contrast_crossover = ((1.0_f64 + 0.05) * (0.0_f64 + 0.05)).sqrt() - 0.05;
        let is_dark = relative_luminance.0 <= contrast_crossover;

        let step = if is_dark { 0.03 } else { -0.03 };

        Some(Self {
            source: hex.to_string(),
            base_100: base_oklch.to_css(),
            base_200: Oklch {
                l: (base_oklch.l + step).clamp(0.0, 1.0),
                c: base_oklch.c + 0.01,
                h: base_oklch.h,
            }
            .to_css(),
            base_300: Oklch {
                l: (base_oklch.l + step * 2.0).clamp(0.0, 1.0),
                c: base_oklch.c + 0.02,
                h: base_oklch.h,
            }
            .to_css(),
            base_content: Oklch {
                l: if is_dark { 0.93 } else { 0.15 },
                c: base_oklch.c * 0.5,
                h: base_oklch.h,
            }
            .to_css(),
        })
    }
}

struct Oklch {
    l: f64,
    c: f64,
    h: f64,
}

impl Oklch {
    fn to_css(&self) -> String {
        format!(
            "oklch({:.1}% {:.3} {:.1})",
            self.l.clamp(0.0, 1.0) * 100.0,
            self.c.clamp(0.0, 0.4),
            self.h
        )
    }
}

impl From<Oklab> for Oklch {
    fn from(oklab: Oklab) -> Self {
        let (lightness, a, b) = (oklab.l, oklab.a, oklab.b);

        let chroma = (a.powi(2) + b.powi(2)).sqrt();
        let mut hue = b.atan2(a).to_degrees();
        if hue < 0.0 {
            hue += 360.0;
        }

        Self {
            l: lightness,
            c: chroma,
            h: hue,
        }
    }
}

struct Oklab {
    l: f64,
    a: f64,
    b: f64,
}

// source: https://bottosson.github.io/posts/oklab/#converting-from-linear-srgb-to-oklab
impl From<&LinearRGB> for Oklab {
    fn from(linear_rgb: &LinearRGB) -> Self {
        let (r, g, b) = (linear_rgb.r, linear_rgb.g, linear_rgb.b);

        // linear sRGB -> LMS
        let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
        let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
        let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

        // LMS -> LMS_ (cube root)
        let (l_, m_, s_) = (l.cbrt(), m.cbrt(), s.cbrt());

        let oklab_l = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
        let oklab_a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
        let oklab_b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

        Self {
            l: oklab_l,
            a: oklab_a,
            b: oklab_b,
        }
    }
}

// The relative luminance
struct Luminance(f64);

impl From<&LinearRGB> for Luminance {
    fn from(linear_rgb: &LinearRGB) -> Self {
        let (r, g, b) = (linear_rgb.r, linear_rgb.g, linear_rgb.b);
        Self(0.2126 * r + 0.7152 * g + 0.0722 * b)
    }
}

struct LinearRGB {
    r: f64,
    g: f64,
    b: f64,
}

impl From<sRGB> for LinearRGB {
    fn from(srgb: sRGB) -> Self {
        let (r, g, b) = (srgb.r, srgb.g, srgb.b);

        let lin = |c: f64| {
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };

        Self {
            r: lin(r),
            g: lin(g),
            b: lin(b),
        }
    }
}

#[allow(non_camel_case_types)]
struct sRGB {
    r: f64,
    g: f64,
    b: f64,
}

// #RRGGBB hex -> 0-255 u8 -> 0-1 f64
impl FromStr for sRGB {
    type Err = &'static str;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let hex = hex.strip_prefix('#').ok_or("missing '#' prefix")?;
        if hex.len() != 6 || !hex.is_ascii() {
            return Err("expected 6 hex digits");
        }

        let channel = |i: usize| {
            u8::from_str_radix(&hex[i..i + 2], 16)
                .map(|c| c as f64 / 255.0)
                .map_err(|_| "invalid hex digit")
        };

        Ok(Self {
            r: channel(0)?,
            g: channel(2)?,
            b: channel(4)?,
        })
    }
}
