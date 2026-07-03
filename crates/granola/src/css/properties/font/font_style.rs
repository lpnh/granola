use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `font-style` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-style)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_style = CssFontStyle::new().content("italic");
///
/// assert_eq!(css_font_style.bake(), "font-style: italic;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font-style: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontStyleRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontStyle<R: FontStyleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: FontStyleRecipe> From<CssFontStyle<R>> for CssDeclaration {
    fn from(css_font_style: CssFontStyle<R>) -> Self {
        Self::new("font-style", css_font_style.bake_recipe().content)
    }
}
