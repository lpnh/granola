use crate::prelude::*;

/// The `inherit` property content recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font = CssFont::from(Inherit);
///
/// assert_eq!(css_font.bake(), "font: inherit;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_align = CssTextAlign::from(Inherit);
///
/// assert_eq!(css_text_align.bake(), "text-align: inherit;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_family = CssFontFamily::from(Inherit);
///
/// assert_eq!(css_font_family.bake(), "font-family: inherit;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_size = CssFontSize::from(Inherit);
///
/// assert_eq!(css_font_size.bake(), "font-size: inherit;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_font_weight = CssFontWeight::from(Inherit);
///
/// assert_eq!(css_font_weight.bake(), "font-weight: inherit;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_color = CssColor::from(Inherit);
///
/// assert_eq!(css_color.bake(), "color: inherit;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration = CssTextDecoration::from(Inherit);
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: inherit;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Inherit;

impl FontRecipe for Inherit {
    recipe_boilerplate!(FontRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl FontFamilyRecipe for Inherit {
    recipe_boilerplate!(FontFamilyRecipe);

    fn content_recipe(content: &mut Self::Content) {
        if content.is_empty() {
            *content = "inherit".into();
        } else {
            *content = format!("{} {}", content, "inherit").into();
        }
    }
}

impl FontSizeRecipe for Inherit {
    recipe_boilerplate!(FontSizeRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl FontWeightRecipe for Inherit {
    recipe_boilerplate!(FontWeightRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl ColorRecipe for Inherit {
    recipe_boilerplate!(ColorRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl BorderColorRecipe for Inherit {
    recipe_boilerplate!(BorderColorRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl TextDecorationRecipe for Inherit {
    recipe_boilerplate!(TextDecorationRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl WebkitTextDecorationRecipe for Inherit {
    recipe_boilerplate!(WebkitTextDecorationRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl WebkitTextSizeAdjustRecipe for Inherit {
    recipe_boilerplate!(WebkitTextSizeAdjustRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl TextAlignRecipe for Inherit {
    recipe_boilerplate!(TextAlignRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl FontFeatureSettingsRecipe for Inherit {
    recipe_boilerplate!(FontFeatureSettingsRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl FontVariationSettingsRecipe for Inherit {
    recipe_boilerplate!(FontVariationSettingsRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}

impl LetterSpacingRecipe for Inherit {
    recipe_boilerplate!(LetterSpacingRecipe);

    fn content_recipe(content: &mut Self::Content) {
        *content = "inherit".into();
    }
}
