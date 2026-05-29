use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `text-align` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-align)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_align: CssTextAlign = CssTextAlign::new("inherit");
///
/// assert_eq!(css_text_align.bake(), "text-align: inherit;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-align: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextAlignRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextAlign<R: TextAlignRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TextAlignRecipe> CssTextAlign<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TextAlignRecipe> From<CssTextAlign<R>> for CssDeclaration {
    fn from(css_text_align: CssTextAlign<R>) -> Self {
        Self::new("text-align", css_text_align.value)
    }
}

impl<R, B> From<CssTextAlign<R>> for CssDeclarationsBlock<B>
where
    R: TextAlignRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_text_align: CssTextAlign<R>) -> Self {
        Self::new().push(css_text_align)
    }
}
