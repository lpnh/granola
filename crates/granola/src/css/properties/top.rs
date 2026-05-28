use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `top` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/top)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_top: CssTop = CssTop::new("-0.5em");
///
/// assert_eq!(css_top.bake(), "top: -0.5em;");
/// ```
///
/// # Askama template
///
/// ```askama
/// top: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TopRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTop<R: TopRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TopRecipe> CssTop<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TopRecipe> From<CssTop<R>> for CssDeclaration {
    fn from(css_top: CssTop<R>) -> Self {
        Self::new("top", css_top.value)
    }
}

impl<R: TopRecipe> From<CssTop<R>> for CssDeclarationsBlock {
    fn from(css_top: CssTop<R>) -> Self {
        Self {
            declarations: vec![css_top.into()],
        }
    }
}
