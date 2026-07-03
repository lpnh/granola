use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `cursor` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/cursor)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_cursor = CssCursor::new().content("pointer");
///
/// assert_eq!(css_cursor.bake(), "cursor: pointer;");
/// ```
///
/// # Askama template
///
/// ```askama
/// cursor: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = CursorRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssCursor<R: CursorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: CursorRecipe> From<CssCursor<R>> for CssDeclaration {
    fn from(css_cursor: CssCursor<R>) -> Self {
        Self::new("cursor", css_cursor.bake_recipe().content)
    }
}
