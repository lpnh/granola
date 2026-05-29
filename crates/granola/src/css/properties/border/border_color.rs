use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `border-color` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/border-color)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_border_color: CssBorderColor = CssBorderColor::new("currentcolor");
///
/// assert_eq!(css_border_color.bake(), "border-color: currentcolor;");
/// ```
///
/// # Askama template
///
/// ```askama
/// border-color: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = BorderColorRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssBorderColor<R: BorderColorRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: BorderColorRecipe> CssBorderColor<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: BorderColorRecipe> From<CssBorderColor<R>> for CssDeclaration {
    fn from(css_border_color: CssBorderColor<R>) -> Self {
        Self::new("border-color", css_border_color.value)
    }
}

impl<R, B> From<CssBorderColor<R>> for CssDeclarationsBlock<B>
where
    R: BorderColorRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_border_color: CssBorderColor<R>) -> Self {
        Self::new().push(css_border_color)
    }
}
