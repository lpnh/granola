use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `auto` property value.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_text_decoration_skip_ink: CssTextDecorationSkipInk<Auto> =
///     CssTextDecorationSkipInk::from_recipe();
///
/// assert_eq!(
///     css_text_decoration_skip_ink.bake(),
///     "text-decoration-skip-ink: auto;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Auto;

impl TextDecorationSkipInkRecipe for Auto {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "auto".into();
    }
}
