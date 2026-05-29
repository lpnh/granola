use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `tab-size` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/tab-size)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_tab_size: CssTabSize = CssTabSize::new("4");
///
/// assert_eq!(css_tab_size.bake(), "tab-size: 4;");
/// ```
///
/// # Askama template
///
/// ```askama
/// tab-size: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TabSizeRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTabSize<R: TabSizeRecipe = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TabSizeRecipe> CssTabSize<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TabSizeRecipe> From<CssTabSize<R>> for CssDeclaration {
    fn from(css_tab_size: CssTabSize<R>) -> Self {
        Self::new("tab-size", css_tab_size.value)
    }
}

impl<R, B> From<CssTabSize<R>> for CssDeclarationsBlock<B>
where
    R: TabSizeRecipe,
    B: DeclarationsBlockRecipe,
{
    fn from(css_tab_size: CssTabSize<R>) -> Self {
        Self::new().push(css_tab_size)
    }
}
