use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `padding-block` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-block)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_padding_block: CssPaddingBlock = CssPaddingBlock::new("0");
///
/// assert_eq!(css_padding_block.bake(), "padding-block: 0;");
/// ```
///
/// # Askama template
///
/// ```askama
/// padding-block: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PaddingBlockRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPaddingBlock<R: PaddingBlockRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: PaddingBlockRecipe> CssPaddingBlock<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: PaddingBlockRecipe> From<CssPaddingBlock<R>> for CssDeclaration {
    fn from(css_padding_block: CssPaddingBlock<R>) -> Self {
        Self::new("padding-block", css_padding_block.value)
    }
}

impl<R, B> From<CssPaddingBlock<R>> for CssDeclarationsBlock<B>
where
    R: PaddingBlockRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_padding_block: CssPaddingBlock<R>) -> Self {
        Self::new().push(css_padding_block)
    }
}
