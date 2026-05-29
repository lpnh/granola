use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `text-size-adjust` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-size-adjust)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_size_adjust: CssTextSizeAdjust = CssTextSizeAdjust::new("none");
///
/// assert_eq!(css_text_size_adjust.bake(), "text-size-adjust: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-size-adjust: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextSizeAdjustRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextSizeAdjust<R: TextSizeAdjustRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TextSizeAdjustRecipe> CssTextSizeAdjust<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TextSizeAdjustRecipe> From<CssTextSizeAdjust<R>> for CssDeclaration {
    fn from(css_text_size_adjust: CssTextSizeAdjust<R>) -> Self {
        Self::new("text-size-adjust", css_text_size_adjust.value)
    }
}

impl<R, B> From<CssTextSizeAdjust<R>> for CssDeclarationsBlock<B>
where
    R: TextSizeAdjustRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_text_size_adjust: CssTextSizeAdjust<R>) -> Self {
        Self::new().push(css_text_size_adjust)
    }
}
