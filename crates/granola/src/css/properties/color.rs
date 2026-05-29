// mod color_interpolation;
// mod color_interpolation_filters;
// mod color_scheme;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/color)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_color: CssColor = CssColor::new("rebeccapurple");
///
/// assert_eq!(css_color.bake(), "color: rebeccapurple;");
/// ```
///
/// # Askama template
///
/// ```askama
/// color: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ColorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssColor<R: ColorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: ColorRecipe> CssColor<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: ColorRecipe> From<CssColor<R>> for CssDeclaration {
    fn from(css_color: CssColor<R>) -> Self {
        Self::new("color", css_color.value)
    }
}

impl<R, B> From<CssColor<R>> for CssDeclarationsBlock<B>
where
    R: ColorRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_color: CssColor<R>) -> Self {
        Self::new().push(css_color)
    }
}
