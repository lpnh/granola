use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `font-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/font-size)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_font_size: CssFontSize = CssFontSize::new("0.875rem");
///
/// assert_eq!(css_font_size.bake(), "font-size: 0.875rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// font-size: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = FontSizeRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssFontSize<R: FontSizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: FontSizeRecipe> CssFontSize<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: FontSizeRecipe> From<CssFontSize<R>> for CssDeclaration {
    fn from(css_font_size: CssFontSize<R>) -> Self {
        Self::new("font-size", css_font_size.value)
    }
}

impl<R, B> From<CssFontSize<R>> for CssDeclarationsBlock<B>
where
    R: FontSizeRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_font_size: CssFontSize<R>) -> Self {
        Self::new().push(css_font_size)
    }
}
