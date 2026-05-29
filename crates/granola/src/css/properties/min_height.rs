use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `min-height` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/min-height)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_min_height: CssMinHeight = CssMinHeight::new("1lh");
///
/// assert_eq!(css_min_height.bake(), "min-height: 1lh;");
/// ```
///
/// # Askama template
///
/// ```askama
/// min-height: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MinHeightRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMinHeight<R: MinHeightRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: MinHeightRecipe> CssMinHeight<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: MinHeightRecipe> From<CssMinHeight<R>> for CssDeclaration {
    fn from(css_min_height: CssMinHeight<R>) -> Self {
        Self::new("min-height", css_min_height.value)
    }
}

impl<R, B> From<CssMinHeight<R>> for CssDeclarationsBlock<B>
where
    R: MinHeightRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_min_height: CssMinHeight<R>) -> Self {
        Self::new().push(css_min_height)
    }
}
