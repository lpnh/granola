use crate::prelude::*;

/// The `center` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_align_items: CssAlignItems<Center> = CssAlignItems::from_cookbook();
///
/// assert_eq!(css_align_items.bake(), "align-items: center;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_justify_content: CssJustifyContent<Center> = CssJustifyContent::from_cookbook();
///
/// assert_eq!(css_justify_content.bake(), "justify-content: center;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Center;

impl AlignItemsRecipe for Center {
    fn content_recipe(content: &mut Self::Content) {
        *content = "center".into();
    }
}

impl JustifyContentRecipe for Center {
    fn content_recipe(content: &mut Self::Content) {
        *content = "center".into();
    }
}

impl TextAlignRecipe for Center {
    fn content_recipe(content: &mut Self::Content) {
        *content = "center".into();
    }
}
