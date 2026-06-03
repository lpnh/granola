use crate::prelude::*;

/// The `none` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border: CssBorder<None> = CssBorder::from_cookbook();
///
/// assert_eq!(css_border.bake(), "border: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_box_shadow: CssBoxShadow<None> = CssBoxShadow::from_cookbook();
///
/// assert_eq!(css_box_shadow.bake(), "box-shadow: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_list_style: CssListStyle<None> = CssListStyle::from_cookbook();
///
/// assert_eq!(css_list_style.bake(), "list-style: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration: CssTextDecoration<None> = CssTextDecoration::from_cookbook();
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_size_adjust: CssTextSizeAdjust<None> = CssTextSizeAdjust::from_cookbook();
///
/// assert_eq!(css_text_size_adjust.bake(), "text-size-adjust: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct None;

impl BorderRecipe for None {
    fn content_recipe(content: &mut Self::Content) {
        *content = "none".into();
    }
}

impl DisplayRecipe for None {
    fn content_recipe(content: &mut Self::Content) {
        *content = "none".into();
    }
}

impl BoxShadowRecipe for None {
    fn content_recipe(content: &mut Self::Content) {
        *content = "none".into();
    }
}

impl ListStyleRecipe for None {
    fn content_recipe(content: &mut Self::Content) {
        *content = "none".into();
    }
}

impl TextDecorationRecipe for None {
    fn content_recipe(content: &mut Self::Content) {
        *content = "none".into();
    }
}

impl TextSizeAdjustRecipe for None {
    fn content_recipe(content: &mut Self::Content) {
        *content = "none".into();
    }
}

impl WebkitTextSizeAdjustRecipe for None {
    fn content_recipe(content: &mut Self::Content) {
        *content = "none".into();
    }
}

impl WebkitTextDecorationRecipe for None {
    fn content_recipe(content: &mut Self::Content) {
        *content = "none".into();
    }
}
