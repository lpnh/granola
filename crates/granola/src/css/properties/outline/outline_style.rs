use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `outline-style` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-style)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline_style = CssOutlineStyle::new().content("0.6em 1.2em");
///
/// assert_eq!(css_outline_style.bake(), "outline-style: 0.6em 1.2em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline-style: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineStyleRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutlineStyle<R: OutlineStyleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: OutlineStyleRecipe> From<CssOutlineStyle<R>> for CssDeclaration {
    fn from(css_outline_style: CssOutlineStyle<R>) -> Self {
        Self::new("outline-style", css_outline_style.bake_recipe().content)
    }
}

impl<R: OutlineStyleRecipe> From<CssOutlineStyle<R>> for CssDeclarationsBlock {
    fn from(css_outline_style: CssOutlineStyle<R>) -> Self {
        Self::new().push(css_outline_style)
    }
}
