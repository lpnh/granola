use std::borrow::Cow;

use crate::prelude::*;

/// The `transparent` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_background_color: CssBackgroundColor<Transparent> = CssBackgroundColor::from_recipe();
///
/// assert_eq!(
///     css_background_color.bake(),
///     "background-color: transparent;"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Transparent;

impl BackgroundColorRecipe for Transparent {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "transparent".into();
    }
}
