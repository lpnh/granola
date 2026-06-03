use crate::recipes::{Flex, Inline};

/// The `inline flex` display value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_display: CssDisplay<InlineFlex> = CssDisplay::from_cookbook();
///
/// assert_eq!(css_display.bake(), "display: inline flex;");
/// ```
pub type InlineFlex = (Inline, Flex);
