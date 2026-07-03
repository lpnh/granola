use crate::prelude::*;

/// The `currentcolor` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border_color = CssBorderColor::from(Currentcolor);
///
/// assert_eq!(css_border_color.bake(), "border-color: currentcolor;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_color = CssColor::from(Currentcolor);
///
/// assert_eq!(css_color.bake(), "color: currentcolor;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Currentcolor;

impl BorderColorRecipe for Currentcolor {
    recipe_boilerplate!(BorderColorRecipe);

    fn content_recipe() -> Self::Content {
        "currentcolor".into()
    }
}

impl ColorRecipe for Currentcolor {
    recipe_boilerplate!(ColorRecipe);

    fn content_recipe() -> Self::Content {
        "currentcolor".into()
    }
}
