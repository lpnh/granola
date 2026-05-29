use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `padding-inline-start` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-inline-start)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_padding_inline_start: CssPaddingInlineStart = CssPaddingInlineStart::new("20px");
///
/// assert_eq!(
///     css_padding_inline_start.bake(),
///     "padding-inline-start: 20px;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// padding-inline-start: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PaddingInlineStartRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPaddingInlineStart<R: PaddingInlineStartRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: PaddingInlineStartRecipe> CssPaddingInlineStart<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: PaddingInlineStartRecipe> From<CssPaddingInlineStart<R>> for CssDeclaration {
    fn from(css_padding_inline_start: CssPaddingInlineStart<R>) -> Self {
        Self::new("padding-inline-start", css_padding_inline_start.value)
    }
}

impl<R, B> From<CssPaddingInlineStart<R>> for CssDeclarationsBlock<B>
where
    R: PaddingInlineStartRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_padding_inline_start: CssPaddingInlineStart<R>) -> Self {
        Self::new().push(css_padding_inline_start)
    }
}
