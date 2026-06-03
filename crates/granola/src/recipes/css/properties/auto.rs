use crate::prelude::*;

/// The `auto` property content recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_height = CssHeight::from(Auto);
///
/// assert_eq!(css_height.bake(), "height: auto;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration_skip_ink = CssTextDecorationSkipInk::from(Auto);
///
/// assert_eq!(
///     css_text_decoration_skip_ink.bake(),
///     "text-decoration-skip-ink: auto;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Auto;

impl HeightRecipe for Auto {
    fn content_recipe(content: &mut Self::Content) {
        *content = "auto".into();
    }
}

impl TextDecorationSkipInkRecipe for Auto {
    fn content_recipe(content: &mut Self::Content) {
        *content = "auto".into();
    }
}

impl OutlineRecipe for Auto {
    fn content_recipe(content: &mut Self::Content) {
        *content = "auto".into();
    }
}
