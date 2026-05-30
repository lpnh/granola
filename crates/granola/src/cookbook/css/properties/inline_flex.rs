use crate::cookbook::*;

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
pub type InlineFlex = (Inline, Flex);
