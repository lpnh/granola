use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `margin-inline-end` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/margin-inline-end)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_margin_inline_end: CssMarginInlineEnd = CssMarginInlineEnd::new("4px");
///
/// assert_eq!(css_margin_inline_end.bake(), "margin-inline-end: 4px;");
/// ```
///
/// # Askama template
///
/// ```askama
/// margin-inline-end: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = MarginInlineEndRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssMarginInlineEnd<R: MarginInlineEndRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: MarginInlineEndRecipe> CssMarginInlineEnd<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: MarginInlineEndRecipe> From<CssMarginInlineEnd<R>> for CssDeclaration {
    fn from(css_margin_inline_end: CssMarginInlineEnd<R>) -> Self {
        Self::new("margin-inline-end", css_margin_inline_end.value)
    }
}

impl<R, B> From<CssMarginInlineEnd<R>> for CssDeclarationsBlock<B>
where
    R: MarginInlineEndRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_margin_inline_end: CssMarginInlineEnd<R>) -> Self {
        Self::new().push(css_margin_inline_end)
    }
}
