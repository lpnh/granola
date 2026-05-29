use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `resize` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/resize)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_resize: CssResize = CssResize::new("vertical");
///
/// assert_eq!(css_resize.bake(), "resize: vertical;");
/// ```
///
/// # Askama template
///
/// ```askama
/// resize: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ResizeRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssResize<R: ResizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: ResizeRecipe> CssResize<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: ResizeRecipe> From<CssResize<R>> for CssDeclaration {
    fn from(css_resize: CssResize<R>) -> Self {
        Self::new("resize", css_resize.value)
    }
}

impl<R, B> From<CssResize<R>> for CssDeclarationsBlock<B>
where
    R: ResizeRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_resize: CssResize<R>) -> Self {
        Self::new().push(css_resize)
    }
}
