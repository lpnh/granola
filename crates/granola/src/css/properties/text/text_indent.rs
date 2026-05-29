use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `text-indent` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-indent)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_indent: CssTextIndent = CssTextIndent::new("0");
///
/// assert_eq!(css_text_indent.bake(), "text-indent: 0;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-indent: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextIndentRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextIndent<R: TextIndentRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TextIndentRecipe> CssTextIndent<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TextIndentRecipe> From<CssTextIndent<R>> for CssDeclaration {
    fn from(css_text_indent: CssTextIndent<R>) -> Self {
        Self::new("text-indent", css_text_indent.value)
    }
}

impl<R, B> From<CssTextIndent<R>> for CssDeclarationsBlock<B>
where
    R: TextIndentRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_text_indent: CssTextIndent<R>) -> Self {
        Self::new().push(css_text_indent)
    }
}
