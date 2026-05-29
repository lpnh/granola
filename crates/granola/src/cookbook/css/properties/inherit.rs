use std::borrow::Cow;

use crate::prelude::*;

/// The `inherit` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_font: CssFont<Inherit> = CssFont::from_recipe();
///
/// assert_eq!(css_font.bake(), "font: inherit;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_text_align: CssTextAlign<Inherit> = CssTextAlign::from_recipe();
///
/// assert_eq!(css_text_align.bake(), "text-align: inherit;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_font_family: CssFontFamily<Inherit> = CssFontFamily::from_recipe();
///
/// assert_eq!(css_font_family.bake(), "font-family: inherit;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_font_size: CssFontSize<Inherit> = CssFontSize::from_recipe();
///
/// assert_eq!(css_font_size.bake(), "font-size: inherit;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_font_weight: CssFontWeight<Inherit> = CssFontWeight::from_recipe();
///
/// assert_eq!(css_font_weight.bake(), "font-weight: inherit;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_color: CssColor<Inherit> = CssColor::from_recipe();
///
/// assert_eq!(css_color.bake(), "color: inherit;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_text_decoration: CssTextDecoration<Inherit> = CssTextDecoration::from_recipe();
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Inherit;

impl FontRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl FontFamilyRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        if value.is_empty() {
            *value = "inherit".into();
        } else {
            *value = format!("{} {}", value, "inherit").into();
        }
    }
}

impl FontSizeRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl FontWeightRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl ColorRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl BorderColorRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl TextDecorationRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl WebkitTextDecorationRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl WebkitTextSizeAdjustRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl TextAlignRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl FontFeatureSettingsRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl FontVariationSettingsRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}

impl LetterSpacingRecipe for Inherit {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "inherit".into();
    }
}
