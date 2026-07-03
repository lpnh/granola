use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `outline-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-width)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline_width = CssOutlineWidth::new().content("2px");
///
/// assert_eq!(css_outline_width.bake(), "outline-width: 2px;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline-width: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineWidthRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutlineWidth<R: OutlineWidthRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: OutlineWidthRecipe> From<CssOutlineWidth<R>> for CssDeclaration {
    fn from(css_outline_width: CssOutlineWidth<R>) -> Self {
        Self::new("outline-width", css_outline_width.bake_recipe().content)
    }
}
