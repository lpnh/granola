use std::borrow::Cow;

use crate::prelude::*;

/// The `none` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_border: CssBorder<None> = CssBorder::from_recipe();
///
/// assert_eq!(css_border.bake(), "border: none;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_list_style: CssListStyle<None> = CssListStyle::from_recipe();
///
/// assert_eq!(css_list_style.bake(), "list-style: none;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_text_decoration: CssTextDecoration<None> = CssTextDecoration::from_recipe();
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: none;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_text_size_adjust: CssTextSizeAdjust<None> = CssTextSizeAdjust::from_recipe();
///
/// assert_eq!(css_text_size_adjust.bake(), "text-size-adjust: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct None;

impl BorderRecipe for None {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "none".into();
    }
}

impl ListStyleRecipe for None {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "none".into();
    }
}

impl TextDecorationRecipe for None {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "none".into();
    }
}

impl TextSizeAdjustRecipe for None {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "none".into();
    }
}
