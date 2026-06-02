use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `interpolate-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/interpolate-size)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_interpolate_size: CssInterpolateSize = CssInterpolateSize::new("allow-keywords");
///
/// assert_eq!(
///     css_interpolate_size.bake(),
///     "interpolate-size: allow-keywords;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// interpolate-size: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = InterpolateSizeRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssInterpolateSize<R: InterpolateSizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: InterpolateSizeRecipe> CssInterpolateSize<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: InterpolateSizeRecipe> From<CssInterpolateSize<R>> for CssDeclaration {
    fn from(css_interpolate_size: CssInterpolateSize<R>) -> Self {
        Self::new("interpolate-size", css_interpolate_size.value)
    }
}

impl<R, B> From<CssInterpolateSize<R>> for CssDeclarationsBlock<B>
where
    R: InterpolateSizeRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_interpolate_size: CssInterpolateSize<R>) -> Self {
        Self::new().push(css_interpolate_size)
    }
}
