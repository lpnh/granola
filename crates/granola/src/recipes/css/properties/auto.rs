use std::borrow::Cow;

use crate::prelude::*;

/// The `auto` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_height: CssHeight<Auto> = CssHeight::from_cookbook();
///
/// assert_eq!(css_height.bake(), "height: auto;");
/// ```
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_text_decoration_skip_ink: CssTextDecorationSkipInk<Auto> =
///     CssTextDecorationSkipInk::from_cookbook();
///
/// assert_eq!(
///     css_text_decoration_skip_ink.bake(),
///     "text-decoration-skip-ink: auto;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Auto;

impl HeightRecipe for Auto {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "auto".into();
    }
}

impl TextDecorationSkipInkRecipe for Auto {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "auto".into();
    }
}

impl OutlineRecipe for Auto {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "auto".into();
    }
}
