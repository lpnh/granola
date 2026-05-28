use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `text-wrap` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/text-wrap)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_text_wrap: CssTextWrap = CssTextWrap::new("balance");
///
/// assert_eq!(css_text_wrap.bake(), "text-wrap: balance;");
/// ```
///
/// # Askama template
///
/// ```askama
/// text-wrap: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TextWrapRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTextWrap<R: TextWrapRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TextWrapRecipe> CssTextWrap<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TextWrapRecipe> From<CssTextWrap<R>> for CssDeclaration {
    fn from(css_text_wrap: CssTextWrap<R>) -> Self {
        Self::new("text-wrap", css_text_wrap.value)
    }
}

impl<R: TextWrapRecipe> From<CssTextWrap<R>> for CssDeclarationsBlock {
    fn from(css_text_wrap: CssTextWrap<R>) -> Self {
        Self {
            declarations: vec![css_text_wrap.into()],
        }
    }
}
