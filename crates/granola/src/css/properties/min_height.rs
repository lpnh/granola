use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `min-height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/min-height)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_min_height = CssMinHeight::new().content("1lh");
///
/// assert_eq!(css_min_height.bake(), "min-height: 1lh;");
/// ```
///
/// # Askama template
///
/// ```askama
/// min-height: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MinHeightRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMinHeight<R: MinHeightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: MinHeightRecipe> From<CssMinHeight<R>> for CssDeclaration {
    fn from(css_min_height: CssMinHeight<R>) -> Self {
        Self::new("min-height", css_min_height.bake_recipe().content)
    }
}

impl<R: MinHeightRecipe> From<CssMinHeight<R>> for CssDeclarationsBlock {
    fn from(css_min_height: CssMinHeight<R>) -> Self {
        Self::new().push(css_min_height)
    }
}
