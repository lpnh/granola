use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `outline-style` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/outline-style)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_outline_style: CssOutlineStyle = CssOutlineStyle::new("0.6em 1.2em");
///
/// assert_eq!(css_outline_style.bake(), "outline-style: 0.6em 1.2em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// outline-style: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = OutlineStyleRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssOutlineStyle<R: OutlineStyleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: OutlineStyleRecipe> CssOutlineStyle<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: OutlineStyleRecipe> From<CssOutlineStyle<R>> for CssDeclaration {
    fn from(css_outline_style: CssOutlineStyle<R>) -> Self {
        Self::new("outline-style", css_outline_style.value)
    }
}

impl<R: OutlineStyleRecipe> From<CssOutlineStyle<R>> for CssDeclarationsBlock {
    fn from(css_outline_style: CssOutlineStyle<R>) -> Self {
        Self {
            declarations: vec![css_outline_style.into()],
        }
    }
}
