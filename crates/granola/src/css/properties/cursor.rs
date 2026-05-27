use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `cursor` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/cursor)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_cursor: CssCursor = CssCursor::new("pointer");
///
/// assert_eq!(css_cursor.bake(), "cursor: pointer;");
/// ```
///
/// # Askama template
///
/// ```askama
/// cursor: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = CursorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssCursor<R: CursorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: CursorRecipe> CssCursor<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: CursorRecipe> From<CssCursor<R>> for CssDeclaration {
    fn from(css_cursor: CssCursor<R>) -> Self {
        Self::new("cursor", css_cursor.value)
    }
}

impl<R: CursorRecipe> From<CssCursor<R>> for CssDeclarationsBlock {
    fn from(css_cursor: CssCursor<R>) -> Self {
        Self {
            declarations: vec![css_cursor.into()],
        }
    }
}
