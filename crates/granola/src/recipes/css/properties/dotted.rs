use crate::prelude::*;

/// The `dotted` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_text_decoration: CssTextDecoration<Dotted> = CssTextDecoration::from_cookbook();
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: dotted;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dotted;

impl TextDecorationRecipe for Dotted {
    recipe_boilerplate!(TextDecorationRecipe);

    fn content_recipe() -> Self::Content {
        "dotted".into()
    }
}

impl WebkitTextDecorationRecipe for Dotted {
    recipe_boilerplate!(WebkitTextDecorationRecipe);

    fn content_recipe() -> Self::Content {
        "dotted".into()
    }
}
