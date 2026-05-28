use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `list-style` shorthand property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/list-style)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_list_style: CssListStyle = CssListStyle::new("none");
///
/// assert_eq!(css_list_style.bake(), "list-style: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// list-style: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = ListStyleRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssListStyle<R: ListStyleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: ListStyleRecipe> CssListStyle<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: ListStyleRecipe> From<CssListStyle<R>> for CssDeclaration {
    fn from(css_list_style: CssListStyle<R>) -> Self {
        Self::new("list-style", css_list_style.value)
    }
}

impl<R: ListStyleRecipe> From<CssListStyle<R>> for CssDeclarationsBlock {
    fn from(css_list_style: CssListStyle<R>) -> Self {
        Self {
            declarations: vec![css_list_style.into()],
        }
    }
}
