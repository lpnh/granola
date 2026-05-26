// mod padding_block;
// mod padding_block-end;
// mod padding_block-start;
// mod padding_bottom;
// mod padding_inline;
// mod padding_inline-end;
// mod padding_inline-start;
// mod padding_left;
// mod padding_right;
// mod padding_top;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `padding` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/padding)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_padding: CssPadding = CssPadding::new("0.6em 1.2em");
///
/// assert_eq!(css_padding.bake(), "padding: 0.6em 1.2em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// padding: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PaddingRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPadding<R: PaddingRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: PaddingRecipe> CssPadding<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: PaddingRecipe> From<CssPadding<R>> for CssDeclaration {
    fn from(css_padding: CssPadding<R>) -> Self {
        Self::new("padding", css_padding.value)
    }
}

impl<R: PaddingRecipe> From<CssPadding<R>> for CssPropertiesList {
    fn from(css_padding: CssPadding<R>) -> Self {
        Self {
            declarations: vec![css_padding.into()],
        }
    }
}
