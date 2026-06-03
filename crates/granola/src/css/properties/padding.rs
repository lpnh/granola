mod padding_block;
pub use padding_block::*;
mod padding_inline_start;
pub use padding_inline_start::*;

// mod padding_block-end;
// mod padding_block-start;
// mod padding_bottom;
// mod padding_inline;
// mod padding_inline-end;
// mod padding_left;
// mod padding_right;
// mod padding_top;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `padding` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_padding = CssPadding::new().content("0.6em 1.2em");
///
/// assert_eq!(css_padding.bake(), "padding: 0.6em 1.2em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// padding: {{ content | kirei(0) }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PaddingRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPadding<R: PaddingRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: PaddingRecipe> From<CssPadding<R>> for CssDeclaration {
    fn from(css_padding: CssPadding<R>) -> Self {
        Self::new("padding", css_padding.bake_recipe().content)
    }
}

impl<R: PaddingRecipe> From<CssPadding<R>> for CssDeclarationsBlock {
    fn from(css_padding: CssPadding<R>) -> Self {
        Self::new().push(css_padding)
    }
}
