use std::borrow::Cow;

use crate::prelude::*;

/// The `center` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_align_items: CssAlignItems<Center> = CssAlignItems::from_recipe();
///
/// assert_eq!(css_align_items.bake(), "align-items: center;");
/// ```
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_justify_content: CssJustifyContent<Center> = CssJustifyContent::from_recipe();
///
/// assert_eq!(css_justify_content.bake(), "justify-content: center;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Center;

impl AlignItemsRecipe for Center {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "center".into();
    }
}

impl JustifyContentRecipe for Center {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "center".into();
    }
}
