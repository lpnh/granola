use std::borrow::Cow;

use crate::prelude::*;

/// The `collapse` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let css_border_collapse: CssBorderCollapse<Collapse> = CssBorderCollapse::from_cookbook();
///
/// assert_eq!(css_border_collapse.bake(), "border-collapse: collapse;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Collapse;

impl BorderCollapseRecipe for Collapse {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "collapse".into();
    }
}
