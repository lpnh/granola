use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `margin-top` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-top)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin_top: CssMarginTop = CssMarginTop::new("1rem");
///
/// assert_eq!(css_margin_top.bake(), "margin-top: 1rem;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin-top: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginTopRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMarginTop<R: MarginTopRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: MarginTopRecipe> CssMarginTop<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: MarginTopRecipe> From<CssMarginTop<R>> for CssDeclaration {
    fn from(css_margin_top: CssMarginTop<R>) -> Self {
        Self::new("margin-top", css_margin_top.value)
    }
}

impl<R, B> From<CssMarginTop<R>> for CssDeclarationsBlock<B>
where
    R: MarginTopRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_margin_top: CssMarginTop<R>) -> Self {
        Self::new().push(css_margin_top)
    }
}
