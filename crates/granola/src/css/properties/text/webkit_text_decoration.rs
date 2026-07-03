use askama::{FastWritable, Template};
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `-webkit-text-decoration` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css = CssWebkitTextDecoration::new().content("none");
///
/// assert_eq!(css.bake(), "-webkit-text-decoration: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// -webkit-text-decoration: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WebkitTextDecorationRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWebkitTextDecoration<R: WebkitTextDecorationRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: WebkitTextDecorationRecipe<Content = Bake>> CssWebkitTextDecoration<R> {
    pub fn fold_in(mut self, value: impl FastWritable) -> Self {
        self.content.fold_in_ws(value);
        self
    }
}

impl<R: WebkitTextDecorationRecipe> From<CssWebkitTextDecoration<R>> for CssDeclaration {
    fn from(css_webkit_text_decoartion: CssWebkitTextDecoration<R>) -> Self {
        Self::new(
            "-webkit-text-decoration",
            css_webkit_text_decoartion.bake_recipe().content,
        )
    }
}
