use askama::Template;
use std::marker::PhantomData;

use crate::{filters, prelude::*};

/// The CSS `padding-block` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding-block)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_padding_block = CssPaddingBlock::new().content("0");
///
/// assert_eq!(css_padding_block.bake(), "padding-block: 0;");
/// ```
///
/// # Askama template
///
/// ```askama
/// padding-block: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PaddingBlockRecipe, content = Bake)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPaddingBlock<R: PaddingBlockRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: PaddingBlockRecipe> From<CssPaddingBlock<R>> for CssDeclaration {
    fn from(css_padding_block: CssPaddingBlock<R>) -> Self {
        Self::new("padding-block", css_padding_block.bake_recipe().content)
    }
}

impl<R: PaddingBlockRecipe> From<CssPaddingBlock<R>> for CssDeclarationsBlock {
    fn from(css_padding_block: CssPaddingBlock<R>) -> Self {
        Self::new().push(css_padding_block)
    }
}
