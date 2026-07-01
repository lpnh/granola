use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `border-top-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-top-width)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_top_width = CssBorderTopWidth::new().content("0.5em");
///
/// assert_eq!(css_border_top_width.bake(), "border-top-width: 0.5em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-top-width: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderTopWidthRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderTopWidth<R: BorderTopWidthRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: BorderTopWidthRecipe> From<CssBorderTopWidth<R>> for CssDeclaration {
    fn from(css_border_top_width: CssBorderTopWidth<R>) -> Self {
        Self::new(
            "border-top-width",
            css_border_top_width.bake_recipe().content,
        )
    }
}

impl<R: BorderTopWidthRecipe> From<CssBorderTopWidth<R>> for CssDeclarationsBlock {
    fn from(css_border_top_width: CssBorderTopWidth<R>) -> Self {
        Self::new().push(css_border_top_width)
    }
}
