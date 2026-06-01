use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/width)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_width: CssWidth = CssWidth::new("100%");
///
/// assert_eq!(css_width.bake(), "width: 100%;");
/// ```
///
/// # Askama template
///
/// ```askama
/// width: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = WidthRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssWidth<R: WidthRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: WidthRecipe> CssWidth<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: WidthRecipe> From<CssWidth<R>> for CssDeclaration {
    fn from(css_width: CssWidth<R>) -> Self {
        Self::new("width", css_width.value)
    }
}

impl<R, B> From<CssWidth<R>> for CssDeclarationsBlock<B>
where
    R: WidthRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_width: CssWidth<R>) -> Self {
        Self::new().push(css_width)
    }
}
