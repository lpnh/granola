use crate::prelude::*;

/// The `center` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_align_items = CssAlignItems::from(Center);
///
/// assert_eq!(css_align_items.bake(), "align-items: center;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_justify_content = CssJustifyContent::from(Center);
///
/// assert_eq!(css_justify_content.bake(), "justify-content: center;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Center;

impl AlignItemsRecipe for Center {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "center".into();
    }
}

impl JustifyContentRecipe for Center {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "center".into();
    }
}

impl TextAlignRecipe for Center {
    recipe_boilerplate!();

    fn content_recipe(content: &mut Self::Content) {
        *content = "center".into();
    }
}
