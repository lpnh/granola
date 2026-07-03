use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/height)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_height = CssHeight::new().content("auto");
///
/// assert_eq!(css_height.bake(), "height: auto;");
/// ```
///
/// # Askama template
///
/// ```askama
/// height: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = HeightRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssHeight<R: HeightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: HeightRecipe> From<CssHeight<R>> for CssDeclaration {
    fn from(css_height: CssHeight<R>) -> Self {
        Self::new("height", css_height.bake_recipe().content)
    }
}
