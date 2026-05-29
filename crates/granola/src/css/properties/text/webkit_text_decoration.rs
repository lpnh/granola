use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `-webkit-text-decoration` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-decoration)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css: CssWebkitTextDecoration = CssWebkitTextDecoration::new("none");
///
/// assert_eq!(css.bake(), "-webkit-text-decoration: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// -webkit-text-decoration: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WebkitTextDecorationRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWebkitTextDecoration<R: WebkitTextDecorationRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: WebkitTextDecorationRecipe> CssWebkitTextDecoration<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: WebkitTextDecorationRecipe> From<CssWebkitTextDecoration<R>> for CssDeclaration {
    fn from(css: CssWebkitTextDecoration<R>) -> Self {
        Self::new("-webkit-text-decoration", css.value)
    }
}

impl<R, B> From<CssWebkitTextDecoration<R>> for CssDeclarationsBlock<B>
where
    R: WebkitTextDecorationRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css: CssWebkitTextDecoration<R>) -> Self {
        Self::new().push(css)
    }
}
