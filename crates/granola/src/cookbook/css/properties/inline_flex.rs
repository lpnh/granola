use crate::{cookbook::*, prelude::*};

/// The `inline flex` display value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_display: CssDisplay<InlineFlex> = CssDisplay::from_recipe();
///
/// assert_eq!(css_display.bake(), "display: inline flex;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InlineFlex;

impl DisplayRecipe for InlineFlex {
    fn content_recipe(content: &mut Self::Content) {
        Inline::content_recipe(content);
        Flex::content_recipe(content);
    }
}
