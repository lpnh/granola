use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `overflow-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/overflow-wrap)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_overflow_wrap: CssOverflowWrap = CssOverflowWrap::new("break-word");
///
/// assert_eq!(css_overflow_wrap.bake(), "overflow-wrap: break-word;");
/// ```
///
/// # Askama template
///
/// ```askama
/// overflow-wrap: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OverflowWrapRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOverflowWrap<R: OverflowWrapRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: OverflowWrapRecipe> CssOverflowWrap<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: OverflowWrapRecipe> From<CssOverflowWrap<R>> for CssDeclaration {
    fn from(css_overflow_wrap: CssOverflowWrap<R>) -> Self {
        Self::new("overflow-wrap", css_overflow_wrap.value)
    }
}

impl<R, B> From<CssOverflowWrap<R>> for CssDeclarationsBlock<B>
where
    R: OverflowWrapRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_overflow_wrap: CssOverflowWrap<R>) -> Self {
        Self::new().push(css_overflow_wrap)
    }
}
