use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `nowrap` property value.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let css_white_space: CssWhiteSpace<Nowrap> = CssWhiteSpace::from_recipe();
///
/// assert_eq!(css_white_space.bake(), "white-space: nowrap;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Nowrap;

impl WhiteSpaceRecipe for Nowrap {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "nowrap".into();
    }
}
