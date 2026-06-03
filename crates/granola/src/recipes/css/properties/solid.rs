use crate::prelude::*;

/// The `solid` property value recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_border = CssBorder::from(Solid).content("0");
///
/// assert_eq!(css_border.bake(), "border: 0 solid;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_outline_style: CssOutlineStyle<Solid> = CssOutlineStyle::from_cookbook();
///
/// assert_eq!(css_outline_style.bake(), "outline-style: solid;");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Solid;

impl BorderRecipe for Solid {
    fn content_recipe(content: &mut Self::Content) {
        if content.is_empty() {
            *content = "solid".into();
        } else {
            *content = format!("{content} {}", "solid").into();
        }
    }
}

impl OutlineStyleRecipe for Solid {
    fn content_recipe(content: &mut Self::Content) {
        *content = "solid".into();
    }
}
