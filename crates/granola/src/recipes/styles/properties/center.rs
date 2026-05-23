use std::borrow::Cow;

use crate::prelude::*;

/// Recipe for the `center` property value.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_align_items: CssAlignItems<Center> = CssAlignItems::from_recipe();
///
/// assert_eq!(css_align_items.bake(),
/// "align-items: center;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_justify_content: CssJustifyContent<Center> = CssJustifyContent::from_recipe();
///
/// assert_eq!(css_justify_content.bake(),
/// "justify-content: center;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Center;

impl AlignItemsTag for Center {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "center".into();
    }
}

impl JustifyContentTag for Center {
    fn value_recipe(value: &mut Cow<'static, str>) {
        *value = "center".into();
    }
}
