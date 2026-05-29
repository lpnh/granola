use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `-webkit-text-size-adjust` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-size-adjust)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css: CssWebkitTextSizeAdjust = CssWebkitTextSizeAdjust::new("100%");
///
/// assert_eq!(css.bake(), "-webkit-text-size-adjust: 100%;");
/// ```
///
/// # Askama template
///
/// ```askama
/// -webkit-text-size-adjust: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WebkitTextSizeAdjustRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWebkitTextSizeAdjust<R: WebkitTextSizeAdjustRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: WebkitTextSizeAdjustRecipe> CssWebkitTextSizeAdjust<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: WebkitTextSizeAdjustRecipe> From<CssWebkitTextSizeAdjust<R>> for CssDeclaration {
    fn from(css: CssWebkitTextSizeAdjust<R>) -> Self {
        Self::new("-webkit-text-size-adjust", css.value)
    }
}

impl<R, B> From<CssWebkitTextSizeAdjust<R>> for CssDeclarationsBlock<B>
where
    R: WebkitTextSizeAdjustRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css: CssWebkitTextSizeAdjust<R>) -> Self {
        Self::new().push(css)
    }
}
