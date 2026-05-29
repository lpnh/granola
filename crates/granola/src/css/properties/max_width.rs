use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `max-width` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/max-width)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_max_width: CssMaxWidth = CssMaxWidth::new("100%");
///
/// assert_eq!(css_max_width.bake(), "max-width: 100%;");
/// ```
///
/// # Askama template
///
/// ```askama
/// max-width: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MaxWidthRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMaxWidth<R: MaxWidthRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: MaxWidthRecipe> CssMaxWidth<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: MaxWidthRecipe> From<CssMaxWidth<R>> for CssDeclaration {
    fn from(css_max_width: CssMaxWidth<R>) -> Self {
        Self::new("max-width", css_max_width.value)
    }
}

impl<R, B> From<CssMaxWidth<R>> for CssDeclarationsBlock<B>
where
    R: MaxWidthRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_max_width: CssMaxWidth<R>) -> Self {
        Self::new().push(css_max_width)
    }
}
