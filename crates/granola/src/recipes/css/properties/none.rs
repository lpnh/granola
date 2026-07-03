use crate::prelude::*;

/// The `none` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border = CssBorder::from(None);
///
/// assert_eq!(css_border.bake(), "border: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_box_shadow = CssBoxShadow::from(None);
///
/// assert_eq!(css_box_shadow.bake(), "box-shadow: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_list_style = CssListStyle::from(None);
///
/// assert_eq!(css_list_style.bake(), "list-style: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration = CssTextDecoration::from(None);
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: none;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_size_adjust = CssTextSizeAdjust::from(None);
///
/// assert_eq!(css_text_size_adjust.bake(), "text-size-adjust: none;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct None;

impl BorderRecipe for None {
    recipe_boilerplate!(BorderRecipe);

    fn content_recipe() -> Self::Content {
        "none".into()
    }
}

impl DisplayRecipe for None {
    recipe_boilerplate!(DisplayRecipe);

    fn content_recipe() -> Self::Content {
        "none".into()
    }
}

impl BoxShadowRecipe for None {
    recipe_boilerplate!(BoxShadowRecipe);

    fn content_recipe() -> Self::Content {
        "none".into()
    }
}

impl ListStyleRecipe for None {
    recipe_boilerplate!(ListStyleRecipe);

    fn content_recipe() -> Self::Content {
        "none".into()
    }
}

impl TextDecorationRecipe for None {
    recipe_boilerplate!(TextDecorationRecipe);

    fn content_recipe() -> Self::Content {
        "none".into()
    }
}

impl TextSizeAdjustRecipe for None {
    recipe_boilerplate!(TextSizeAdjustRecipe);

    fn content_recipe() -> Self::Content {
        "none".into()
    }
}

impl WebkitTextSizeAdjustRecipe for None {
    recipe_boilerplate!(WebkitTextSizeAdjustRecipe);

    fn content_recipe() -> Self::Content {
        "none".into()
    }
}

impl WebkitTextDecorationRecipe for None {
    recipe_boilerplate!(WebkitTextDecorationRecipe);

    fn content_recipe() -> Self::Content {
        "none".into()
    }
}
