use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `text-decoration` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_decoration = CssTextDecoration::new().content("none");
///
/// assert_eq!(css_text_decoration.bake(), "text-decoration: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-decoration: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextDecorationRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextDecoration<R: TextDecorationRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TextDecorationRecipe> From<CssTextDecoration<R>> for CssDeclaration {
    fn from(css_text_decoration: CssTextDecoration<R>) -> Self {
        Self::new("text-decoration", css_text_decoration.bake_recipe().content)
    }
}

impl<R: TextDecorationRecipe> From<CssTextDecoration<R>> for CssDeclarationsBlock {
    fn from(css_text_decoration: CssTextDecoration<R>) -> Self {
        Self::new().push(css_text_decoration)
    }
}
