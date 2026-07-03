use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `outline-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-color)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline_color = CssOutlineColor::new().content("inherit");
///
/// assert_eq!(css_outline_color.bake(), "outline-color: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline-color: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineColorRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutlineColor<R: OutlineColorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: OutlineColorRecipe> From<CssOutlineColor<R>> for CssDeclaration {
    fn from(css_outline_color: CssOutlineColor<R>) -> Self {
        Self::new("outline-color", css_outline_color.bake_recipe().content)
    }
}
